// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

use super::b_tree_u_32_type::BTreeU32;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct DeleteFromBtreeU32Args {
    pub rows: Vec<BTreeU32>,
}

impl From<DeleteFromBtreeU32Args> for super::Reducer {
    fn from(args: DeleteFromBtreeU32Args) -> Self {
        Self::DeleteFromBtreeU32 { rows: args.rows }
    }
}

impl __sdk::InModule for DeleteFromBtreeU32Args {
    type Module = super::RemoteModule;
}

pub struct DeleteFromBtreeU32CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `delete_from_btree_u32`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait delete_from_btree_u_32 {
    /// Request that the remote module invoke the reducer `delete_from_btree_u32` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_delete_from_btree_u_32`] callbacks.
    fn delete_from_btree_u_32(&self, rows: Vec<BTreeU32>) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `delete_from_btree_u32`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`DeleteFromBtreeU32CallbackId`] can be passed to [`Self::remove_on_delete_from_btree_u_32`]
    /// to cancel the callback.
    fn on_delete_from_btree_u_32(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &Vec<BTreeU32>) + Send + 'static,
    ) -> DeleteFromBtreeU32CallbackId;
    /// Cancel a callback previously registered by [`Self::on_delete_from_btree_u_32`],
    /// causing it not to run in the future.
    fn remove_on_delete_from_btree_u_32(&self, callback: DeleteFromBtreeU32CallbackId);
}

impl delete_from_btree_u_32 for super::RemoteReducers {
    fn delete_from_btree_u_32(&self, rows: Vec<BTreeU32>) -> __sdk::Result<()> {
        self.imp
            .call_reducer("delete_from_btree_u32", DeleteFromBtreeU32Args { rows })
    }
    fn on_delete_from_btree_u_32(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &Vec<BTreeU32>) + Send + 'static,
    ) -> DeleteFromBtreeU32CallbackId {
        DeleteFromBtreeU32CallbackId(self.imp.on_reducer(
            "delete_from_btree_u32",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::DeleteFromBtreeU32 { rows },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, rows)
            }),
        ))
    }
    fn remove_on_delete_from_btree_u_32(&self, callback: DeleteFromBtreeU32CallbackId) {
        self.imp.remove_on_reducer("delete_from_btree_u32", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `delete_from_btree_u32`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_delete_from_btree_u_32 {
    /// Set the call-reducer flags for the reducer `delete_from_btree_u32` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn delete_from_btree_u_32(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_delete_from_btree_u_32 for super::SetReducerFlags {
    fn delete_from_btree_u_32(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("delete_from_btree_u32", flags);
    }
}
