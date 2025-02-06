// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct InsertVecF64Args {
    pub f: Vec<f64>,
}

impl From<InsertVecF64Args> for super::Reducer {
    fn from(args: InsertVecF64Args) -> Self {
        Self::InsertVecF64 { f: args.f }
    }
}

impl __sdk::InModule for InsertVecF64Args {
    type Module = super::RemoteModule;
}

pub struct InsertVecF64CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_vec_f64`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_vec_f_64 {
    /// Request that the remote module invoke the reducer `insert_vec_f64` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_vec_f_64`] callbacks.
    fn insert_vec_f_64(&self, f: Vec<f64>) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_vec_f64`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertVecF64CallbackId`] can be passed to [`Self::remove_on_insert_vec_f_64`]
    /// to cancel the callback.
    fn on_insert_vec_f_64(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &Vec<f64>) + Send + 'static,
    ) -> InsertVecF64CallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_vec_f_64`],
    /// causing it not to run in the future.
    fn remove_on_insert_vec_f_64(&self, callback: InsertVecF64CallbackId);
}

impl insert_vec_f_64 for super::RemoteReducers {
    fn insert_vec_f_64(&self, f: Vec<f64>) -> __sdk::Result<()> {
        self.imp.call_reducer("insert_vec_f64", InsertVecF64Args { f })
    }
    fn on_insert_vec_f_64(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &Vec<f64>) + Send + 'static,
    ) -> InsertVecF64CallbackId {
        InsertVecF64CallbackId(self.imp.on_reducer(
            "insert_vec_f64",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::InsertVecF64 { f },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, f)
            }),
        ))
    }
    fn remove_on_insert_vec_f_64(&self, callback: InsertVecF64CallbackId) {
        self.imp.remove_on_reducer("insert_vec_f64", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_vec_f64`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_insert_vec_f_64 {
    /// Set the call-reducer flags for the reducer `insert_vec_f64` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn insert_vec_f_64(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_vec_f_64 for super::SetReducerFlags {
    fn insert_vec_f_64(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("insert_vec_f64", flags);
    }
}
