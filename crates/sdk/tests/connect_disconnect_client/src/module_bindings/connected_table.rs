// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::connected_type::Connected;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `Connected`.
///
/// Obtain a handle from the [`ConnectedTableAccess::connected`] method on [`super::RemoteTables`],
/// like `ctx.db.connected()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.connected().on_insert(...)`.
pub struct ConnectedTableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<Connected>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `Connected`.
///
/// Implemented for [`super::RemoteTables`].
pub trait ConnectedTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`ConnectedTableHandle`], which mediates access to the table `Connected`.
    fn connected(&self) -> ConnectedTableHandle<'_>;
}

impl ConnectedTableAccess for super::RemoteTables {
    fn connected(&self) -> ConnectedTableHandle<'_> {
        ConnectedTableHandle {
            imp: self.imp.get_table::<Connected>("Connected"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct ConnectedInsertCallbackId(__sdk::callbacks::CallbackId);
pub struct ConnectedDeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for ConnectedTableHandle<'ctx> {
    type Row = Connected;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = Connected> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = ConnectedInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> ConnectedInsertCallbackId {
        ConnectedInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: ConnectedInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = ConnectedDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> ConnectedDeleteCallbackId {
        ConnectedDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: ConnectedDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    deletes: Vec<__ws::EncodedValue>,
    inserts: Vec<__ws::EncodedValue>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<Connected>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(deletes, inserts)
        .context("Failed to parse table update for table \"Connected\"")
}
