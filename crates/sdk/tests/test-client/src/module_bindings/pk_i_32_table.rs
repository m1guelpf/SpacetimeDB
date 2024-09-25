// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::pk_i_32_type::PkI32;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `PkI32`.
///
/// Obtain a handle from the [`PkI32TableAccess::pk_i_32`] method on [`super::RemoteTables`],
/// like `ctx.db.pk_i_32()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.pk_i_32().on_insert(...)`.
pub struct PkI32TableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<PkI32>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `PkI32`.
///
/// Implemented for [`super::RemoteTables`].
pub trait PkI32TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`PkI32TableHandle`], which mediates access to the table `PkI32`.
    fn pk_i_32(&self) -> PkI32TableHandle<'_>;
}

impl PkI32TableAccess for super::RemoteTables {
    fn pk_i_32(&self) -> PkI32TableHandle<'_> {
        PkI32TableHandle {
            imp: self.imp.get_table::<PkI32>("PkI32"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct PkI32InsertCallbackId(__sdk::callbacks::CallbackId);
pub struct PkI32DeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for PkI32TableHandle<'ctx> {
    type Row = PkI32;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = PkI32> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = PkI32InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PkI32InsertCallbackId {
        PkI32InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: PkI32InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = PkI32DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PkI32DeleteCallbackId {
        PkI32DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: PkI32DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

pub struct PkI32UpdateCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::TableWithPrimaryKey for PkI32TableHandle<'ctx> {
    type UpdateCallbackId = PkI32UpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> PkI32UpdateCallbackId {
        PkI32UpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: PkI32UpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    deletes: Vec<__ws::EncodedValue>,
    inserts: Vec<__ws::EncodedValue>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<PkI32>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_with_primary_key::<i32>(deletes, inserts, |row: &PkI32| {
        &row.n
    })
    .context("Failed to parse table update for table \"PkI32\"")
}

/// Access to the `n` unique index on the table `PkI32`,
/// which allows point queries on the field of the same name
/// via the [`PkI32NUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.pk_i_32().n().find(...)`.
pub struct PkI32NUnique<'ctx> {
    imp: __sdk::client_cache::UniqueConstraint<PkI32, i32>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> PkI32TableHandle<'ctx> {
    /// Get a handle on the `n` unique index on the table `PkI32`.
    pub fn n(&self) -> PkI32NUnique<'ctx> {
        PkI32NUnique {
            imp: self.imp.get_unique_constraint::<i32>("n", |row| &row.n),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> PkI32NUnique<'ctx> {
    /// Find the subscribed row whose `n` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &i32) -> Option<PkI32> {
        self.imp.find(col_val)
    }
}
