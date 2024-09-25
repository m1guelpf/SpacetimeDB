// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::vec_f_32_type::VecF32;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `VecF32`.
///
/// Obtain a handle from the [`VecF32TableAccess::vec_f_32`] method on [`super::RemoteTables`],
/// like `ctx.db.vec_f_32()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.vec_f_32().on_insert(...)`.
pub struct VecF32TableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<VecF32>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `VecF32`.
///
/// Implemented for [`super::RemoteTables`].
pub trait VecF32TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`VecF32TableHandle`], which mediates access to the table `VecF32`.
    fn vec_f_32(&self) -> VecF32TableHandle<'_>;
}

impl VecF32TableAccess for super::RemoteTables {
    fn vec_f_32(&self) -> VecF32TableHandle<'_> {
        VecF32TableHandle {
            imp: self.imp.get_table::<VecF32>("VecF32"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct VecF32InsertCallbackId(__sdk::callbacks::CallbackId);
pub struct VecF32DeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for VecF32TableHandle<'ctx> {
    type Row = VecF32;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = VecF32> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = VecF32InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecF32InsertCallbackId {
        VecF32InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: VecF32InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = VecF32DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecF32DeleteCallbackId {
        VecF32DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: VecF32DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    deletes: Vec<__ws::EncodedValue>,
    inserts: Vec<__ws::EncodedValue>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<VecF32>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(deletes, inserts)
        .context("Failed to parse table update for table \"VecF32\"")
}
