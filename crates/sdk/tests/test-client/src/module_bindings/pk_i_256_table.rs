// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::pk_i_256_type::PkI256;
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

/// Table handle for the table `pk_i256`.
///
/// Obtain a handle from the [`PkI256TableAccess::pk_i_256`] method on [`super::RemoteTables`],
/// like `ctx.db.pk_i_256()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.pk_i_256().on_insert(...)`.
pub struct PkI256TableHandle<'ctx> {
    imp: __sdk::TableHandle<PkI256>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `pk_i256`.
///
/// Implemented for [`super::RemoteTables`].
pub trait PkI256TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`PkI256TableHandle`], which mediates access to the table `pk_i256`.
    fn pk_i_256(&self) -> PkI256TableHandle<'_>;
}

impl PkI256TableAccess for super::RemoteTables {
    fn pk_i_256(&self) -> PkI256TableHandle<'_> {
        PkI256TableHandle {
            imp: self.imp.get_table::<PkI256>("pk_i256"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct PkI256InsertCallbackId(__sdk::CallbackId);
pub struct PkI256DeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for PkI256TableHandle<'ctx> {
    type Row = PkI256;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = PkI256> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = PkI256InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PkI256InsertCallbackId {
        PkI256InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: PkI256InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = PkI256DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PkI256DeleteCallbackId {
        PkI256DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: PkI256DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<PkI256>("pk_i256");
    _table.add_unique_constraint::<__sats::i256>("n", |row| &row.n)
}
pub struct PkI256UpdateCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::TableWithPrimaryKey for PkI256TableHandle<'ctx> {
    type UpdateCallbackId = PkI256UpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> PkI256UpdateCallbackId {
        PkI256UpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: PkI256UpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::TableUpdate<PkI256>> {
    __sdk::TableUpdate::parse_table_update_with_primary_key::<__sats::i256>(raw_updates, |row: &PkI256| &row.n)
        .context("Failed to parse table update for table \"pk_i256\"")
}

/// Access to the `n` unique index on the table `pk_i256`,
/// which allows point queries on the field of the same name
/// via the [`PkI256NUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.pk_i_256().n().find(...)`.
pub struct PkI256NUnique<'ctx> {
    imp: __sdk::UniqueConstraintHandle<PkI256, __sats::i256>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> PkI256TableHandle<'ctx> {
    /// Get a handle on the `n` unique index on the table `pk_i256`.
    pub fn n(&self) -> PkI256NUnique<'ctx> {
        PkI256NUnique {
            imp: self.imp.get_unique_constraint::<__sats::i256>("n"),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> PkI256NUnique<'ctx> {
    /// Find the subscribed row whose `n` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &__sats::i256) -> Option<PkI256> {
        self.imp.find(col_val)
    }
}
