// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::unique_i_256_type::UniqueI256;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

pub struct UniqueI256TableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<UniqueI256>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
pub trait UniqueI256TableAccess {
    #[allow(non_snake_case)]
    fn unique_i_256(&self) -> UniqueI256TableHandle<'_>;
}

impl UniqueI256TableAccess for super::RemoteTables {
    fn unique_i_256(&self) -> UniqueI256TableHandle<'_> {
        UniqueI256TableHandle {
            imp: self.imp.get_table::<UniqueI256>("UniqueI256"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct UniqueI256InsertCallbackId(__sdk::callbacks::CallbackId);
pub struct UniqueI256DeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for UniqueI256TableHandle<'ctx> {
    type Row = UniqueI256;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = UniqueI256> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = UniqueI256InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueI256InsertCallbackId {
        UniqueI256InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: UniqueI256InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = UniqueI256DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueI256DeleteCallbackId {
        UniqueI256DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: UniqueI256DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

pub(super) fn parse_table_update(
    deletes: Vec<__ws::EncodedValue>,
    inserts: Vec<__ws::EncodedValue>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<UniqueI256>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(deletes, inserts)
        .context("Failed to parse table update for table \"UniqueI256\"")
}

pub struct UniqueI256NUnique<'ctx> {
    imp: __sdk::client_cache::UniqueConstraint<UniqueI256, __sats::i256>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> UniqueI256TableHandle<'ctx> {
    pub fn n(&self) -> UniqueI256NUnique<'ctx> {
        UniqueI256NUnique {
            imp: self.imp.get_unique_constraint::<__sats::i256>("n", |row| &row.n),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> UniqueI256NUnique<'ctx> {
    pub fn find(&self, col_val: &__sats::i256) -> Option<UniqueI256> {
        self.imp.find(col_val)
    }
}
