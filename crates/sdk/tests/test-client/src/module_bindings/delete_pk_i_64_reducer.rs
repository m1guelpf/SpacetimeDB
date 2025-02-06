// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct DeletePkI64Args {
    pub n: i64,
}

impl From<DeletePkI64Args> for super::Reducer {
    fn from(args: DeletePkI64Args) -> Self {
        Self::DeletePkI64 { n: args.n }
    }
}

impl __sdk::InModule for DeletePkI64Args {
    type Module = super::RemoteModule;
}

pub struct DeletePkI64CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `delete_pk_i64`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait delete_pk_i_64 {
    /// Request that the remote module invoke the reducer `delete_pk_i64` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_delete_pk_i_64`] callbacks.
    fn delete_pk_i_64(&self, n: i64) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `delete_pk_i64`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`DeletePkI64CallbackId`] can be passed to [`Self::remove_on_delete_pk_i_64`]
    /// to cancel the callback.
    fn on_delete_pk_i_64(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &i64) + Send + 'static,
    ) -> DeletePkI64CallbackId;
    /// Cancel a callback previously registered by [`Self::on_delete_pk_i_64`],
    /// causing it not to run in the future.
    fn remove_on_delete_pk_i_64(&self, callback: DeletePkI64CallbackId);
}

impl delete_pk_i_64 for super::RemoteReducers {
    fn delete_pk_i_64(&self, n: i64) -> __sdk::Result<()> {
        self.imp.call_reducer("delete_pk_i64", DeletePkI64Args { n })
    }
    fn on_delete_pk_i_64(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &i64) + Send + 'static,
    ) -> DeletePkI64CallbackId {
        DeletePkI64CallbackId(self.imp.on_reducer(
            "delete_pk_i64",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::DeletePkI64 { n },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, n)
            }),
        ))
    }
    fn remove_on_delete_pk_i_64(&self, callback: DeletePkI64CallbackId) {
        self.imp.remove_on_reducer("delete_pk_i64", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `delete_pk_i64`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_delete_pk_i_64 {
    /// Set the call-reducer flags for the reducer `delete_pk_i64` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn delete_pk_i_64(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_delete_pk_i_64 for super::SetReducerFlags {
    fn delete_pk_i_64(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("delete_pk_i64", flags);
    }
}
