// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{
	self as __sdk,
	__lib,
	__sats,
	__ws,
};
use super::one_connection_id_type::OneConnectionId;

/// Table handle for the table `one_connection_id`.
///
/// Obtain a handle from the [`OneConnectionIdTableAccess::one_connection_id`] method on [`super::RemoteTables`],
/// like `ctx.db.one_connection_id()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.one_connection_id().on_insert(...)`.
pub struct OneConnectionIdTableHandle<'ctx> {
    imp: __sdk::TableHandle<OneConnectionId>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `one_connection_id`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OneConnectionIdTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OneConnectionIdTableHandle`], which mediates access to the table `one_connection_id`.
    fn one_connection_id(&self) -> OneConnectionIdTableHandle<'_>;
}

impl OneConnectionIdTableAccess for super::RemoteTables {
    fn one_connection_id(&self) -> OneConnectionIdTableHandle<'_> {
        OneConnectionIdTableHandle {
            imp: self.imp.get_table::<OneConnectionId>("one_connection_id"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OneConnectionIdInsertCallbackId(__sdk::CallbackId);
pub struct OneConnectionIdDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for OneConnectionIdTableHandle<'ctx> {
    type Row = OneConnectionId;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 { self.imp.count() }
    fn iter(&self) -> impl Iterator<Item = OneConnectionId> + '_ { self.imp.iter() }

    type InsertCallbackId = OneConnectionIdInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneConnectionIdInsertCallbackId {
        OneConnectionIdInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OneConnectionIdInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OneConnectionIdDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneConnectionIdDeleteCallbackId {
        OneConnectionIdDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OneConnectionIdDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {

        let _table = client_cache.get_or_make_table::<OneConnectionId>("one_connection_id");
}
#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<OneConnectionId>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .map_err(|e| {
             __sdk::InternalError::failed_parse(
                "TableUpdate<OneConnectionId>",
                "TableUpdate",
            ).with_cause(e).into()
        })
}
