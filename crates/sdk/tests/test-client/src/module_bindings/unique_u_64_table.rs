// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use super::unique_u_64_type::UniqueU64;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `unique_u64`.
///
/// Obtain a handle from the [`UniqueU64TableAccess::unique_u_64`] method on [`super::RemoteTables`],
/// like `ctx.db.unique_u_64()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_u_64().on_insert(...)`.
pub struct UniqueU64TableHandle<'ctx> {
    imp: __sdk::TableHandle<UniqueU64>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `unique_u64`.
///
/// Implemented for [`super::RemoteTables`].
pub trait UniqueU64TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`UniqueU64TableHandle`], which mediates access to the table `unique_u64`.
    fn unique_u_64(&self) -> UniqueU64TableHandle<'_>;
}

impl UniqueU64TableAccess for super::RemoteTables {
    fn unique_u_64(&self) -> UniqueU64TableHandle<'_> {
        UniqueU64TableHandle {
            imp: self.imp.get_table::<UniqueU64>("unique_u64"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct UniqueU64InsertCallbackId(__sdk::CallbackId);
pub struct UniqueU64DeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for UniqueU64TableHandle<'ctx> {
    type Row = UniqueU64;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = UniqueU64> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = UniqueU64InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueU64InsertCallbackId {
        UniqueU64InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: UniqueU64InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = UniqueU64DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueU64DeleteCallbackId {
        UniqueU64DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: UniqueU64DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<UniqueU64>("unique_u64");
    _table.add_unique_constraint::<u64>("n", |row| &row.n);
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<UniqueU64>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<UniqueU64>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}

/// Access to the `n` unique index on the table `unique_u64`,
/// which allows point queries on the field of the same name
/// via the [`UniqueU64NUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_u_64().n().find(...)`.
pub struct UniqueU64NUnique<'ctx> {
    imp: __sdk::UniqueConstraintHandle<UniqueU64, u64>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> UniqueU64TableHandle<'ctx> {
    /// Get a handle on the `n` unique index on the table `unique_u64`.
    pub fn n(&self) -> UniqueU64NUnique<'ctx> {
        UniqueU64NUnique {
            imp: self.imp.get_unique_constraint::<u64>("n"),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> UniqueU64NUnique<'ctx> {
    /// Find the subscribed row whose `n` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &u64) -> Option<UniqueU64> {
        self.imp.find(col_val)
    }
}
