//! Low-level WebSocket plumbing.
//!
//! This module is internal, and may incompatibly change without warning.

use std::sync::Arc;

use bytes::Bytes;
use futures::{SinkExt, StreamExt as _, TryStreamExt};
use futures_channel::mpsc;
use http::uri::{InvalidUri, Scheme, Uri};
use spacetimedb_client_api_messages::websocket::{
    brotli_decompress, gzip_decompress, BsatnFormat, Compression, SERVER_MSG_COMPRESSION_TAG_BROTLI,
    SERVER_MSG_COMPRESSION_TAG_GZIP, SERVER_MSG_COMPRESSION_TAG_NONE,
};
use spacetimedb_client_api_messages::websocket::{ClientMessage, ServerMessage};
use spacetimedb_lib::{bsatn, ConnectionId};
use thiserror::Error;
use tokio::task::JoinHandle;
use tokio::{net::TcpStream, runtime};
use tokio_tungstenite::{
    connect_async_with_config,
    tungstenite::client::IntoClientRequest,
    tungstenite::protocol::{Message as WebSocketMessage, WebSocketConfig},
    MaybeTlsStream, WebSocketStream,
};

#[derive(Error, Debug, Clone)]
pub enum UriError {
    #[error("Unknown URI scheme {scheme}, expected http, https, ws or wss")]
    UnknownUriScheme { scheme: String },

    #[error("Expected a URI without a query part, but found {query}")]
    UnexpectedQuery { query: String },

    #[error(transparent)]
    InvalidUri {
        // `Arc` is required for `Self: Clone`, as `http::uri::InvalidUri: !Clone`.
        source: Arc<http::uri::InvalidUri>,
    },

    #[error(transparent)]
    InvalidUriParts {
        // `Arc` is required for `Self: Clone`, as `http::uri::InvalidUriParts: !Clone`.
        source: Arc<http::uri::InvalidUriParts>,
    },
}

#[derive(Error, Debug, Clone)]
pub enum WsError {
    #[error(transparent)]
    UriError(#[from] UriError),

    #[error("Error in WebSocket connection with {uri}: {source}")]
    Tungstenite {
        uri: Uri,
        #[source]
        // `Arc` is required for `Self: Clone`, as `tungstenite::Error: !Clone`.
        source: Arc<tokio_tungstenite::tungstenite::Error>,
    },

    #[error("Received empty raw message, but valid messages always start with a one-byte compression flag")]
    EmptyMessage,

    #[error("Failed to deserialize WebSocket message: {source}")]
    DeserializeMessage {
        #[source]
        source: bsatn::DecodeError,
    },

    #[error("Failed to decompress WebSocket message with {scheme}: {source}")]
    Decompress {
        scheme: &'static str,
        #[source]
        // `Arc` is required for `Self: Clone`, as `std::io::Error: !Clone`.
        source: Arc<std::io::Error>,
    },

    #[error("Unrecognized compression scheme: {scheme:#x}")]
    UnknownCompressionScheme { scheme: u8 },
}

pub(crate) struct WsConnection {
    sock: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

fn parse_scheme(scheme: Option<Scheme>) -> Result<Scheme, UriError> {
    Ok(match scheme {
        Some(s) => match s.as_str() {
            "ws" | "wss" => s,
            "http" => "ws".parse().unwrap(),
            "https" => "wss".parse().unwrap(),
            unknown_scheme => {
                return Err(UriError::UnknownUriScheme {
                    scheme: unknown_scheme.into(),
                })
            }
        },
        None => "ws".parse().unwrap(),
    })
}

#[derive(Clone, Copy, Default)]
pub(crate) struct WsParams {
    pub compression: Compression,
    pub light: bool,
}

fn make_uri(host: Uri, db_name: &str, connection_id: ConnectionId, params: WsParams) -> Result<Uri, UriError> {
    let mut parts = host.into_parts();
    let scheme = parse_scheme(parts.scheme.take())?;
    parts.scheme = Some(scheme);
    let mut path = if let Some(path_and_query) = parts.path_and_query {
        if let Some(query) = path_and_query.query() {
            return Err(UriError::UnexpectedQuery { query: query.into() });
        }
        path_and_query.path().to_string()
    } else {
        "/".to_string()
    };

    // Normalize the path, ensuring it ends with `/`.
    if !path.ends_with('/') {
        path.push('/');
    }

    path.push_str("database/subscribe/");
    path.push_str(db_name);

    // Provide the connection ID.
    path.push_str("?connection_id=");
    path.push_str(&connection_id.to_hex());

    // Specify the desired compression for host->client replies.
    match params.compression {
        Compression::None => path.push_str("&compression=None"),
        Compression::Gzip => path.push_str("&compression=Gzip"),
        // The host uses the same default as the sdk,
        // but in case this changes, we prefer to be explicit now.
        Compression::Brotli => path.push_str("&compression=Brotli"),
    };

    // Specify the `light` mode if requested.
    if params.light {
        path.push_str("&light=true");
    }

    parts.path_and_query = Some(path.parse().map_err(|source: InvalidUri| UriError::InvalidUri {
        source: Arc::new(source),
    })?);
    Uri::from_parts(parts).map_err(|source| UriError::InvalidUriParts {
        source: Arc::new(source),
    })
}

// Tungstenite doesn't offer an interface to specify a WebSocket protocol, which frankly
// seems like a pretty glaring omission in its API. In order to insert our own protocol
// header, we manually the `Request` constructed by
// `tungstenite::IntoClientRequest::into_client_request`.

// TODO: `core` uses [Hyper](https://docs.rs/hyper/latest/hyper/) as its HTTP library
//       rather than having Tungstenite manage its own connections. Should this library do
//       the same?

fn make_request(
    host: Uri,
    db_name: &str,
    token: Option<&str>,
    connection_id: ConnectionId,
    params: WsParams,
) -> Result<http::Request<()>, WsError> {
    let uri = make_uri(host, db_name, connection_id, params)?;
    let mut req = IntoClientRequest::into_client_request(uri.clone()).map_err(|source| WsError::Tungstenite {
        uri,
        source: Arc::new(source),
    })?;
    request_insert_protocol_header(&mut req);
    request_insert_auth_header(&mut req, token);
    Ok(req)
}

fn request_add_header(req: &mut http::Request<()>, key: &'static str, val: http::header::HeaderValue) {
    let _prev = req.headers_mut().insert(key, val);
    debug_assert!(_prev.is_none(), "HttpRequest already had {:?} header {:?}", key, _prev,);
}

const PROTOCOL_HEADER_KEY: &str = "Sec-WebSocket-Protocol";
const PROTOCOL_HEADER_VALUE: &str = "v1.bsatn.spacetimedb";

fn request_insert_protocol_header(req: &mut http::Request<()>) {
    request_add_header(
        req,
        PROTOCOL_HEADER_KEY,
        http::header::HeaderValue::from_static(PROTOCOL_HEADER_VALUE),
    );
}

const AUTH_HEADER_KEY: &str = "Authorization";

fn request_insert_auth_header(req: &mut http::Request<()>, token: Option<&str>) {
    // TODO: figure out how the token is supposed to be encoded in the request
    if let Some(token) = token {
        use base64::Engine;

        let auth_bytes = format!("token:{}", token);
        let encoded = base64::prelude::BASE64_STANDARD.encode(auth_bytes);
        let auth_header_val = format!("Basic {}", encoded);
        request_add_header(
            req,
            AUTH_HEADER_KEY,
            auth_header_val
                .try_into()
                .expect("Failed to convert token to http HeaderValue"),
        )
    };
}

impl WsConnection {
    pub(crate) async fn connect(
        host: Uri,
        db_name: &str,
        token: Option<&str>,
        connection_id: ConnectionId,
        params: WsParams,
    ) -> Result<Self, WsError> {
        let req = make_request(host, db_name, token, connection_id, params)?;

        // Grab the URI for error-reporting.
        let uri = req.uri().clone();

        let (sock, _): (WebSocketStream<MaybeTlsStream<TcpStream>>, _) = connect_async_with_config(
            req,
            // TODO(kim): In order to be able to replicate module WASM blobs,
            // `cloud-next` cannot have message / frame size limits. That's
            // obviously a bad default for all other clients, though.
            Some(WebSocketConfig {
                max_frame_size: None,
                max_message_size: None,
                ..WebSocketConfig::default()
            }),
            false,
        )
        .await
        .map_err(|source| WsError::Tungstenite {
            uri,
            source: Arc::new(source),
        })?;
        Ok(WsConnection { sock })
    }

    pub(crate) fn parse_response(bytes: &[u8]) -> Result<ServerMessage<BsatnFormat>, WsError> {
        let (compression, bytes) = bytes.split_first().ok_or(WsError::EmptyMessage)?;

        Ok(match *compression {
            SERVER_MSG_COMPRESSION_TAG_NONE => {
                bsatn::from_slice(bytes).map_err(|source| WsError::DeserializeMessage { source })?
            }
            SERVER_MSG_COMPRESSION_TAG_BROTLI => {
                bsatn::from_slice(&brotli_decompress(bytes).map_err(|source| WsError::Decompress {
                    scheme: "brotli",
                    source: Arc::new(source),
                })?)
                .map_err(|source| WsError::DeserializeMessage { source })?
            }
            SERVER_MSG_COMPRESSION_TAG_GZIP => {
                bsatn::from_slice(&gzip_decompress(bytes).map_err(|source| WsError::Decompress {
                    scheme: "gzip",
                    source: Arc::new(source),
                })?)
                .map_err(|source| WsError::DeserializeMessage { source })?
            }
            c => {
                return Err(WsError::UnknownCompressionScheme { scheme: c });
            }
        })
    }

    pub(crate) fn encode_message(msg: ClientMessage<Bytes>) -> WebSocketMessage {
        WebSocketMessage::Binary(bsatn::to_vec(&msg).unwrap())
    }

    fn maybe_log_error<T, U: std::fmt::Debug>(cause: &str, res: std::result::Result<T, U>) {
        if let Err(e) = res {
            log::warn!("{}: {:?}", cause, e);
        }
    }

    async fn message_loop(
        mut self,
        incoming_messages: mpsc::UnboundedSender<ServerMessage<BsatnFormat>>,
        outgoing_messages: mpsc::UnboundedReceiver<ClientMessage<Bytes>>,
    ) {
        let mut outgoing_messages = Some(outgoing_messages);
        loop {
            tokio::select! {
                incoming = self.sock.try_next() => match incoming {
                    Err(tokio_tungstenite::tungstenite::error::Error::ConnectionClosed) | Ok(None) => break,

                    Err(e) => Self::maybe_log_error::<(), _>(
                        "Error reading message from read WebSocket stream",
                        Err(e),
                    ),

                    Ok(Some(WebSocketMessage::Binary(bytes))) => {
                        match Self::parse_response(&bytes) {
                            Err(e) => Self::maybe_log_error::<(), _>(
                                "Error decoding WebSocketMessage::Binary payload",
                                Err(e),
                            ),
                            Ok(msg) => Self::maybe_log_error(
                                "Error sending decoded message to incoming_messages queue",
                                incoming_messages.unbounded_send(msg),
                            ),
                        }
                    }

                    Ok(Some(WebSocketMessage::Ping(_))) => {}

                    Ok(Some(other)) => log::warn!("Unexpected WebSocket message {:?}", other),
                },

                // this is stupid. we want to handle the channel close *once*, and then disable this branch
                Some(outgoing) = async { Some(outgoing_messages.as_mut()?.next().await) } => match outgoing {
                    Some(outgoing) => {
                        let msg = Self::encode_message(outgoing);
                        Self::maybe_log_error(
                            "Error sending outgoing message",
                                self.sock.send(msg).await,
                        );
                    }
                    None => {
                        Self::maybe_log_error("Error sending close frame", SinkExt::close(&mut self.sock).await);
                        outgoing_messages = None;
                    }
                },
            }
        }
    }

    pub(crate) fn spawn_message_loop(
        self,
        runtime: &runtime::Handle,
    ) -> (
        JoinHandle<()>,
        mpsc::UnboundedReceiver<ServerMessage<BsatnFormat>>,
        mpsc::UnboundedSender<ClientMessage<Bytes>>,
    ) {
        let (outgoing_send, outgoing_recv) = mpsc::unbounded();
        let (incoming_send, incoming_recv) = mpsc::unbounded();
        let handle = runtime.spawn(self.message_loop(incoming_send, outgoing_recv));
        (handle, incoming_recv, outgoing_send)
    }
}
