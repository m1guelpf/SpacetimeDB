// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct UpdatePkI64 {
    pub n: i64,
    pub data: i32,
}

impl __sdk::InModule for UpdatePkI64 {
    type Module = super::RemoteModule;
}

pub struct UpdatePkI64CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `update_pk_i64`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait update_pk_i_64 {
    /// Request that the remote module invoke the reducer `update_pk_i64` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_update_pk_i_64`] callbacks.
    fn update_pk_i_64(&self, n: i64, data: i32) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `update_pk_i64`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`UpdatePkI64CallbackId`] can be passed to [`Self::remove_on_update_pk_i_64`]
    /// to cancel the callback.
    fn on_update_pk_i_64(
        &self,
        callback: impl FnMut(&super::EventContext, &i64, &i32) + Send + 'static,
    ) -> UpdatePkI64CallbackId;
    /// Cancel a callback previously registered by [`Self::on_update_pk_i_64`],
    /// causing it not to run in the future.
    fn remove_on_update_pk_i_64(&self, callback: UpdatePkI64CallbackId);
}

impl update_pk_i_64 for super::RemoteReducers {
    fn update_pk_i_64(&self, n: i64, data: i32) -> __anyhow::Result<()> {
        self.imp.call_reducer("update_pk_i64", UpdatePkI64 { n, data })
    }
    fn on_update_pk_i_64(
        &self,
        mut callback: impl FnMut(&super::EventContext, &i64, &i32) + Send + 'static,
    ) -> UpdatePkI64CallbackId {
        UpdatePkI64CallbackId(self.imp.on_reducer::<UpdatePkI64>(
            "update_pk_i64",
            Box::new(move |ctx: &super::EventContext, args: &UpdatePkI64| callback(ctx, &args.n, &args.data)),
        ))
    }
    fn remove_on_update_pk_i_64(&self, callback: UpdatePkI64CallbackId) {
        self.imp.remove_on_reducer::<UpdatePkI64>("update_pk_i64", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `update_pk_i64`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_update_pk_i_64 {
    /// Set the call-reducer flags for the reducer `update_pk_i64` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn update_pk_i_64(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_update_pk_i_64 for super::SetReducerFlags {
    fn update_pk_i_64(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("update_pk_i64", flags);
    }
}
