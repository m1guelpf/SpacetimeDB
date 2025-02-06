// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{
	self as __sdk,
	__lib,
	__sats,
	__ws,
};


#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct InsertCallerUniqueConnectionIdArgs {
    pub data: i32,
}

impl From<InsertCallerUniqueConnectionIdArgs> for super::Reducer {
    fn from(args: InsertCallerUniqueConnectionIdArgs) -> Self {
        Self::InsertCallerUniqueConnectionId {
            data: args.data,
}
}
}

impl __sdk::InModule for InsertCallerUniqueConnectionIdArgs {
    type Module = super::RemoteModule;
}

pub struct InsertCallerUniqueConnectionIdCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_caller_unique_connection_id`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_caller_unique_connection_id {
    /// Request that the remote module invoke the reducer `insert_caller_unique_connection_id` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_caller_unique_connection_id`] callbacks.
    fn insert_caller_unique_connection_id(&self, data: i32,
) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_caller_unique_connection_id`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertCallerUniqueConnectionIdCallbackId`] can be passed to [`Self::remove_on_insert_caller_unique_connection_id`]
    /// to cancel the callback.
    fn on_insert_caller_unique_connection_id(&self, callback: impl FnMut(&super::ReducerEventContext, &i32, ) + Send + 'static) -> InsertCallerUniqueConnectionIdCallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_caller_unique_connection_id`],
    /// causing it not to run in the future.
    fn remove_on_insert_caller_unique_connection_id(&self, callback: InsertCallerUniqueConnectionIdCallbackId);
}

impl insert_caller_unique_connection_id for super::RemoteReducers {
    fn insert_caller_unique_connection_id(&self, data: i32,
) -> __sdk::Result<()> {
        self.imp.call_reducer("insert_caller_unique_connection_id", InsertCallerUniqueConnectionIdArgs { data,  })
    }
    fn on_insert_caller_unique_connection_id(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &i32, ) + Send + 'static,
    ) -> InsertCallerUniqueConnectionIdCallbackId {
        InsertCallerUniqueConnectionIdCallbackId(self.imp.on_reducer(
            "insert_caller_unique_connection_id",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event: __sdk::ReducerEvent {
                        reducer: super::Reducer::InsertCallerUniqueConnectionId {
                            data, 
                        },
                        ..
                    },
                    ..
                } = ctx else { unreachable!() };
                callback(ctx, data, )
            }),
        ))
    }
    fn remove_on_insert_caller_unique_connection_id(&self, callback: InsertCallerUniqueConnectionIdCallbackId) {
        self.imp.remove_on_reducer("insert_caller_unique_connection_id", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_caller_unique_connection_id`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_insert_caller_unique_connection_id {
    /// Set the call-reducer flags for the reducer `insert_caller_unique_connection_id` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn insert_caller_unique_connection_id(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_caller_unique_connection_id for super::SetReducerFlags {
    fn insert_caller_unique_connection_id(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("insert_caller_unique_connection_id", flags);
    }
}

