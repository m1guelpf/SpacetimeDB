// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::vec_i_8_type::VecI8;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `VecI8`.
///
/// Obtain a handle from the [`VecI8TableAccess::vec_i_8`] method on [`super::RemoteTables`],
/// like `ctx.db.vec_i_8()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.vec_i_8().on_insert(...)`.
pub struct VecI8TableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<VecI8>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `VecI8`.
///
/// Implemented for [`super::RemoteTables`].
pub trait VecI8TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`VecI8TableHandle`], which mediates access to the table `VecI8`.
    fn vec_i_8(&self) -> VecI8TableHandle<'_>;
}

impl VecI8TableAccess for super::RemoteTables {
    fn vec_i_8(&self) -> VecI8TableHandle<'_> {
        VecI8TableHandle {
            imp: self.imp.get_table::<VecI8>("VecI8"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct VecI8InsertCallbackId(__sdk::callbacks::CallbackId);
pub struct VecI8DeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for VecI8TableHandle<'ctx> {
    type Row = VecI8;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = VecI8> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = VecI8InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecI8InsertCallbackId {
        VecI8InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: VecI8InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = VecI8DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecI8DeleteCallbackId {
        VecI8DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: VecI8DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    deletes: Vec<__ws::EncodedValue>,
    inserts: Vec<__ws::EncodedValue>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<VecI8>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(deletes, inserts)
        .context("Failed to parse table update for table \"VecI8\"")
}
