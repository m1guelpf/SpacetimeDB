// Any `#[doc(hidden)]` modules or uses are public because code generated by the CLI's codegen
// references them, but users should not.

#[doc(hidden)]
pub mod callbacks;
#[doc(hidden)]
pub mod client_cache;
#[doc(hidden)]
pub mod db_connection;
#[doc(hidden)]
pub mod spacetime_module;
#[doc(hidden)]
pub mod subscription;
#[doc(hidden)]
pub mod websocket;

#[doc(hidden)]
pub use http;
#[doc(hidden)]
pub use spacetimedb_client_api_messages::websocket as ws_messages;
#[doc(hidden)]
pub use spacetimedb_sats as sats;

pub mod db_context;
pub mod event;
pub mod table;

pub use db_connection::DbConnectionBuilder;
pub use db_context::DbContext;
pub use event::{Event, ReducerEvent, Status};
pub use table::{Table, TableWithPrimaryKey};

// We re-export `spacetimedb_lib` so the cli codegen can reference it through us, rather
// than requiring downstream users to depend on it explicitly.
// TODO: determine if this should be `#[doc(hidden)]`
pub use spacetimedb_lib::{self, Address, Identity, ScheduleAt};
// Ditto re-exporing `log`.
// TODO: determine if this should be `#[doc(hidden)]`.
pub use log;
// Ditto re-exporting `anyhow`. This is not `#[doc(hidden)]`, because users may want to
// refer to results we return as `anyhow::Result`.
// TODO: determine if we should re-export anything.
pub use anyhow;
