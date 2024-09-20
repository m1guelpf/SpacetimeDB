// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::unique_string_type::UniqueString;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

pub struct UniqueStringTableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<UniqueString>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
pub trait UniqueStringTableAccess {
    #[allow(non_snake_case)]
    fn unique_string(&self) -> UniqueStringTableHandle<'_>;
}

impl UniqueStringTableAccess for super::RemoteTables {
    fn unique_string(&self) -> UniqueStringTableHandle<'_> {
        UniqueStringTableHandle {
            imp: self.imp.get_table::<UniqueString>("UniqueString"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct UniqueStringInsertCallbackId(__sdk::callbacks::CallbackId);
pub struct UniqueStringDeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for UniqueStringTableHandle<'ctx> {
    type Row = UniqueString;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = UniqueString> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = UniqueStringInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueStringInsertCallbackId {
        UniqueStringInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: UniqueStringInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = UniqueStringDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueStringDeleteCallbackId {
        UniqueStringDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: UniqueStringDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

pub(super) fn parse_table_update(
    deletes: Vec<__ws::EncodedValue>,
    inserts: Vec<__ws::EncodedValue>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<UniqueString>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(deletes, inserts)
        .context("Failed to parse table update for table \"UniqueString\"")
}

pub struct UniqueStringSUnique<'ctx> {
    imp: __sdk::client_cache::UniqueConstraint<UniqueString, String>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> UniqueStringTableHandle<'ctx> {
    pub fn s(&self) -> UniqueStringSUnique<'ctx> {
        UniqueStringSUnique {
            imp: self.imp.get_unique_constraint::<String>("s", |row| &row.s),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> UniqueStringSUnique<'ctx> {
    pub fn find(&self, col_val: &String) -> Option<UniqueString> {
        self.imp.find(col_val)
    }
}
