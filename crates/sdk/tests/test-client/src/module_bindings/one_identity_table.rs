// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::one_identity_type::OneIdentity;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `OneIdentity`.
///
/// Obtain a handle from the [`OneIdentityTableAccess::one_identity`] method on [`super::RemoteTables`],
/// like `ctx.db.one_identity()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.one_identity().on_insert(...)`.
pub struct OneIdentityTableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<OneIdentity>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `OneIdentity`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OneIdentityTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OneIdentityTableHandle`], which mediates access to the table `OneIdentity`.
    fn one_identity(&self) -> OneIdentityTableHandle<'_>;
}

impl OneIdentityTableAccess for super::RemoteTables {
    fn one_identity(&self) -> OneIdentityTableHandle<'_> {
        OneIdentityTableHandle {
            imp: self.imp.get_table::<OneIdentity>("OneIdentity"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OneIdentityInsertCallbackId(__sdk::callbacks::CallbackId);
pub struct OneIdentityDeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for OneIdentityTableHandle<'ctx> {
    type Row = OneIdentity;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OneIdentity> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OneIdentityInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneIdentityInsertCallbackId {
        OneIdentityInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OneIdentityInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OneIdentityDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneIdentityDeleteCallbackId {
        OneIdentityDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OneIdentityDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    deletes: Vec<__ws::EncodedValue>,
    inserts: Vec<__ws::EncodedValue>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<OneIdentity>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(deletes, inserts)
        .context("Failed to parse table update for table \"OneIdentity\"")
}
