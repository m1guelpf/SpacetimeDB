// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::one_unit_struct_type::OneUnitStruct;
use super::unit_struct_type::UnitStruct;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

pub struct OneUnitStructTableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<OneUnitStruct>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
pub trait OneUnitStructTableAccess {
    #[allow(non_snake_case)]
    fn one_unit_struct(&self) -> OneUnitStructTableHandle<'_>;
}

impl OneUnitStructTableAccess for super::RemoteTables {
    fn one_unit_struct(&self) -> OneUnitStructTableHandle<'_> {
        OneUnitStructTableHandle {
            imp: self.imp.get_table::<OneUnitStruct>("OneUnitStruct"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OneUnitStructInsertCallbackId(__sdk::callbacks::CallbackId);
pub struct OneUnitStructDeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for OneUnitStructTableHandle<'ctx> {
    type Row = OneUnitStruct;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OneUnitStruct> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OneUnitStructInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneUnitStructInsertCallbackId {
        OneUnitStructInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OneUnitStructInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OneUnitStructDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneUnitStructDeleteCallbackId {
        OneUnitStructDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OneUnitStructDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

pub(super) fn parse_table_update(
    deletes: Vec<__ws::EncodedValue>,
    inserts: Vec<__ws::EncodedValue>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<OneUnitStruct>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(deletes, inserts)
        .context("Failed to parse table update for table \"OneUnitStruct\"")
}
