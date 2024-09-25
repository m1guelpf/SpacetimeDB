// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::option_identity_type::OptionIdentity;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `OptionIdentity`.
///
/// Obtain a handle from the [`OptionIdentityTableAccess::option_identity`] method on [`super::RemoteTables`],
/// like `ctx.db.option_identity()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.option_identity().on_insert(...)`.
pub struct OptionIdentityTableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<OptionIdentity>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `OptionIdentity`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OptionIdentityTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OptionIdentityTableHandle`], which mediates access to the table `OptionIdentity`.
    fn option_identity(&self) -> OptionIdentityTableHandle<'_>;
}

impl OptionIdentityTableAccess for super::RemoteTables {
    fn option_identity(&self) -> OptionIdentityTableHandle<'_> {
        OptionIdentityTableHandle {
            imp: self.imp.get_table::<OptionIdentity>("OptionIdentity"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OptionIdentityInsertCallbackId(__sdk::callbacks::CallbackId);
pub struct OptionIdentityDeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for OptionIdentityTableHandle<'ctx> {
    type Row = OptionIdentity;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OptionIdentity> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OptionIdentityInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OptionIdentityInsertCallbackId {
        OptionIdentityInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OptionIdentityInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OptionIdentityDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OptionIdentityDeleteCallbackId {
        OptionIdentityDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OptionIdentityDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    deletes: Vec<__ws::EncodedValue>,
    inserts: Vec<__ws::EncodedValue>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<OptionIdentity>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(deletes, inserts)
        .context("Failed to parse table update for table \"OptionIdentity\"")
}
