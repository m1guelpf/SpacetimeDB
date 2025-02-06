// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{
	self as __sdk,
	__lib,
	__sats,
	__ws,
};
use super::one_every_vec_struct_type::OneEveryVecStruct;
use super::every_vec_struct_type::EveryVecStruct;

/// Table handle for the table `one_every_vec_struct`.
///
/// Obtain a handle from the [`OneEveryVecStructTableAccess::one_every_vec_struct`] method on [`super::RemoteTables`],
/// like `ctx.db.one_every_vec_struct()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.one_every_vec_struct().on_insert(...)`.
pub struct OneEveryVecStructTableHandle<'ctx> {
    imp: __sdk::TableHandle<OneEveryVecStruct>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `one_every_vec_struct`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OneEveryVecStructTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OneEveryVecStructTableHandle`], which mediates access to the table `one_every_vec_struct`.
    fn one_every_vec_struct(&self) -> OneEveryVecStructTableHandle<'_>;
}

impl OneEveryVecStructTableAccess for super::RemoteTables {
    fn one_every_vec_struct(&self) -> OneEveryVecStructTableHandle<'_> {
        OneEveryVecStructTableHandle {
            imp: self.imp.get_table::<OneEveryVecStruct>("one_every_vec_struct"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OneEveryVecStructInsertCallbackId(__sdk::CallbackId);
pub struct OneEveryVecStructDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for OneEveryVecStructTableHandle<'ctx> {
    type Row = OneEveryVecStruct;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 { self.imp.count() }
    fn iter(&self) -> impl Iterator<Item = OneEveryVecStruct> + '_ { self.imp.iter() }

    type InsertCallbackId = OneEveryVecStructInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneEveryVecStructInsertCallbackId {
        OneEveryVecStructInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OneEveryVecStructInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OneEveryVecStructDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneEveryVecStructDeleteCallbackId {
        OneEveryVecStructDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OneEveryVecStructDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {

        let _table = client_cache.get_or_make_table::<OneEveryVecStruct>("one_every_vec_struct");
}
#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<OneEveryVecStruct>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .map_err(|e| {
             __sdk::InternalError::failed_parse(
                "TableUpdate<OneEveryVecStruct>",
                "TableUpdate",
            ).with_cause(e).into()
        })
}
