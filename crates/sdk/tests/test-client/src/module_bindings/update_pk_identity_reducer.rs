// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct UpdatePkIdentity {
    pub i: __sdk::Identity,
    pub data: i32,
}

impl __sdk::spacetime_module::InModule for UpdatePkIdentity {
    type Module = super::RemoteModule;
}

pub struct UpdatePkIdentityCallbackId(__sdk::callbacks::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `update_pk_identity`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait update_pk_identity {
    /// Request that the remote module invoke the reducer `update_pk_identity` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_update_pk_identity`] callbacks.
    fn update_pk_identity(&self, i: __sdk::Identity, data: i32) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `update_pk_identity`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`UpdatePkIdentityCallbackId`] can be passed to [`Self::remove_on_update_pk_identity`]
    /// to cancel the callback.
    fn on_update_pk_identity(
        &self,
        callback: impl FnMut(&super::EventContext, &__sdk::Identity, &i32) + Send + 'static,
    ) -> UpdatePkIdentityCallbackId;
    /// Cancel a callback previously registered by [`Self::on_update_pk_identity`],
    /// causing it not to run in the future.
    fn remove_on_update_pk_identity(&self, callback: UpdatePkIdentityCallbackId);
}

impl update_pk_identity for super::RemoteReducers {
    fn update_pk_identity(&self, i: __sdk::Identity, data: i32) -> __anyhow::Result<()> {
        self.imp
            .call_reducer("update_pk_identity", UpdatePkIdentity { i, data })
    }
    fn on_update_pk_identity(
        &self,
        mut callback: impl FnMut(&super::EventContext, &__sdk::Identity, &i32) + Send + 'static,
    ) -> UpdatePkIdentityCallbackId {
        UpdatePkIdentityCallbackId(self.imp.on_reducer::<UpdatePkIdentity>(
            "update_pk_identity",
            Box::new(move |ctx: &super::EventContext, args: &UpdatePkIdentity| callback(ctx, &args.i, &args.data)),
        ))
    }
    fn remove_on_update_pk_identity(&self, callback: UpdatePkIdentityCallbackId) {
        self.imp
            .remove_on_reducer::<UpdatePkIdentity>("update_pk_identity", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `update_pk_identity`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_update_pk_identity {
    /// Set the call-reducer flags for the reducer `update_pk_identity` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn update_pk_identity(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_update_pk_identity for super::SetReducerFlags {
    fn update_pk_identity(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("update_pk_identity", flags);
    }
}
