// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct DeletePkU256Args {
    pub n: __sats::u256,
}

impl From<DeletePkU256Args> for super::Reducer {
    fn from(args: DeletePkU256Args) -> Self {
        Self::DeletePkU256 { n: args.n }
    }
}

impl __sdk::InModule for DeletePkU256Args {
    type Module = super::RemoteModule;
}

pub struct DeletePkU256CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `delete_pk_u256`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait delete_pk_u_256 {
    /// Request that the remote module invoke the reducer `delete_pk_u256` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_delete_pk_u_256`] callbacks.
    fn delete_pk_u_256(&self, n: __sats::u256) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `delete_pk_u256`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`DeletePkU256CallbackId`] can be passed to [`Self::remove_on_delete_pk_u_256`]
    /// to cancel the callback.
    fn on_delete_pk_u_256(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &__sats::u256) + Send + 'static,
    ) -> DeletePkU256CallbackId;
    /// Cancel a callback previously registered by [`Self::on_delete_pk_u_256`],
    /// causing it not to run in the future.
    fn remove_on_delete_pk_u_256(&self, callback: DeletePkU256CallbackId);
}

impl delete_pk_u_256 for super::RemoteReducers {
    fn delete_pk_u_256(&self, n: __sats::u256) -> __sdk::Result<()> {
        self.imp.call_reducer("delete_pk_u256", DeletePkU256Args { n })
    }
    fn on_delete_pk_u_256(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &__sats::u256) + Send + 'static,
    ) -> DeletePkU256CallbackId {
        DeletePkU256CallbackId(self.imp.on_reducer(
            "delete_pk_u256",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::DeletePkU256 { n },
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
    fn remove_on_delete_pk_u_256(&self, callback: DeletePkU256CallbackId) {
        self.imp.remove_on_reducer("delete_pk_u256", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `delete_pk_u256`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_delete_pk_u_256 {
    /// Set the call-reducer flags for the reducer `delete_pk_u256` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn delete_pk_u_256(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_delete_pk_u_256 for super::SetReducerFlags {
    fn delete_pk_u_256(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("delete_pk_u256", flags);
    }
}
