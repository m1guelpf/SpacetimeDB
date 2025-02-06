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
pub(super) struct InsertOneI128Args {
    pub n: i128,
}

impl From<InsertOneI128Args> for super::Reducer {
    fn from(args: InsertOneI128Args) -> Self {
        Self::InsertOneI128 {
            n: args.n,
}
}
}

impl __sdk::InModule for InsertOneI128Args {
    type Module = super::RemoteModule;
}

pub struct InsertOneI128CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_one_i128`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_one_i_128 {
    /// Request that the remote module invoke the reducer `insert_one_i128` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_one_i_128`] callbacks.
    fn insert_one_i_128(&self, n: i128,
) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_one_i128`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertOneI128CallbackId`] can be passed to [`Self::remove_on_insert_one_i_128`]
    /// to cancel the callback.
    fn on_insert_one_i_128(&self, callback: impl FnMut(&super::ReducerEventContext, &i128, ) + Send + 'static) -> InsertOneI128CallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_one_i_128`],
    /// causing it not to run in the future.
    fn remove_on_insert_one_i_128(&self, callback: InsertOneI128CallbackId);
}

impl insert_one_i_128 for super::RemoteReducers {
    fn insert_one_i_128(&self, n: i128,
) -> __sdk::Result<()> {
        self.imp.call_reducer("insert_one_i128", InsertOneI128Args { n,  })
    }
    fn on_insert_one_i_128(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &i128, ) + Send + 'static,
    ) -> InsertOneI128CallbackId {
        InsertOneI128CallbackId(self.imp.on_reducer(
            "insert_one_i128",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event: __sdk::ReducerEvent {
                        reducer: super::Reducer::InsertOneI128 {
                            n, 
                        },
                        ..
                    },
                    ..
                } = ctx else { unreachable!() };
                callback(ctx, n, )
            }),
        ))
    }
    fn remove_on_insert_one_i_128(&self, callback: InsertOneI128CallbackId) {
        self.imp.remove_on_reducer("insert_one_i128", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_one_i128`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_insert_one_i_128 {
    /// Set the call-reducer flags for the reducer `insert_one_i128` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn insert_one_i_128(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_one_i_128 for super::SetReducerFlags {
    fn insert_one_i_128(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("insert_one_i128", flags);
    }
}

