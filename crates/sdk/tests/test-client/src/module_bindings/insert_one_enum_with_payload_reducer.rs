// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

use super::enum_with_payload_type::EnumWithPayload;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct InsertOneEnumWithPayload {
    pub e: EnumWithPayload,
}

impl __sdk::spacetime_module::InModule for InsertOneEnumWithPayload {
    type Module = super::RemoteModule;
}

pub struct InsertOneEnumWithPayloadCallbackId(__sdk::callbacks::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_one_enum_with_payload`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_one_enum_with_payload {
    /// Request that the remote module invoke the reducer `insert_one_enum_with_payload` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_one_enum_with_payload`] callbacks.
    fn insert_one_enum_with_payload(&self, e: EnumWithPayload) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_one_enum_with_payload`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertOneEnumWithPayloadCallbackId`] can be passed to [`Self::remove_on_insert_one_enum_with_payload`]
    /// to cancel the callback.
    fn on_insert_one_enum_with_payload(
        &self,
        callback: impl FnMut(&super::EventContext, &EnumWithPayload) + Send + 'static,
    ) -> InsertOneEnumWithPayloadCallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_one_enum_with_payload`],
    /// causing it not to run in the future.
    fn remove_on_insert_one_enum_with_payload(&self, callback: InsertOneEnumWithPayloadCallbackId);
}

impl insert_one_enum_with_payload for super::RemoteReducers {
    fn insert_one_enum_with_payload(&self, e: EnumWithPayload) -> __anyhow::Result<()> {
        self.imp
            .call_reducer("insert_one_enum_with_payload", InsertOneEnumWithPayload { e })
    }
    fn on_insert_one_enum_with_payload(
        &self,
        mut callback: impl FnMut(&super::EventContext, &EnumWithPayload) + Send + 'static,
    ) -> InsertOneEnumWithPayloadCallbackId {
        InsertOneEnumWithPayloadCallbackId(self.imp.on_reducer::<InsertOneEnumWithPayload>(
            "insert_one_enum_with_payload",
            Box::new(move |ctx: &super::EventContext, args: &InsertOneEnumWithPayload| callback(ctx, &args.e)),
        ))
    }
    fn remove_on_insert_one_enum_with_payload(&self, callback: InsertOneEnumWithPayloadCallbackId) {
        self.imp
            .remove_on_reducer::<InsertOneEnumWithPayload>("insert_one_enum_with_payload", callback.0)
    }
}
