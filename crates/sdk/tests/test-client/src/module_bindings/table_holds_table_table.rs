// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::one_u_8_type::OneU8;
use super::table_holds_table_type::TableHoldsTable;
use super::vec_u_8_type::VecU8;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

pub struct TableHoldsTableTableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<TableHoldsTable>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
pub trait TableHoldsTableTableAccess {
    #[allow(non_snake_case)]
    fn table_holds_table(&self) -> TableHoldsTableTableHandle<'_>;
}

impl TableHoldsTableTableAccess for super::RemoteTables {
    fn table_holds_table(&self) -> TableHoldsTableTableHandle<'_> {
        TableHoldsTableTableHandle {
            imp: self.imp.get_table::<TableHoldsTable>("TableHoldsTable"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct TableHoldsTableInsertCallbackId(__sdk::callbacks::CallbackId);
pub struct TableHoldsTableDeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for TableHoldsTableTableHandle<'ctx> {
    type Row = TableHoldsTable;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = TableHoldsTable> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = TableHoldsTableInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> TableHoldsTableInsertCallbackId {
        TableHoldsTableInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: TableHoldsTableInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = TableHoldsTableDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> TableHoldsTableDeleteCallbackId {
        TableHoldsTableDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: TableHoldsTableDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

pub(super) fn parse_table_update(
    deletes: Vec<__ws::EncodedValue>,
    inserts: Vec<__ws::EncodedValue>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<TableHoldsTable>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(deletes, inserts)
        .context("Failed to parse table update for table \"TableHoldsTable\"")
}
