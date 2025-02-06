// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{
	self as __sdk,
	__lib,
	__sats,
	__ws,
};
use super::vec_f_64_type::VecF64;

/// Table handle for the table `vec_f64`.
///
/// Obtain a handle from the [`VecF64TableAccess::vec_f_64`] method on [`super::RemoteTables`],
/// like `ctx.db.vec_f_64()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.vec_f_64().on_insert(...)`.
pub struct VecF64TableHandle<'ctx> {
    imp: __sdk::TableHandle<VecF64>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `vec_f64`.
///
/// Implemented for [`super::RemoteTables`].
pub trait VecF64TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`VecF64TableHandle`], which mediates access to the table `vec_f64`.
    fn vec_f_64(&self) -> VecF64TableHandle<'_>;
}

impl VecF64TableAccess for super::RemoteTables {
    fn vec_f_64(&self) -> VecF64TableHandle<'_> {
        VecF64TableHandle {
            imp: self.imp.get_table::<VecF64>("vec_f64"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct VecF64InsertCallbackId(__sdk::CallbackId);
pub struct VecF64DeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for VecF64TableHandle<'ctx> {
    type Row = VecF64;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 { self.imp.count() }
    fn iter(&self) -> impl Iterator<Item = VecF64> + '_ { self.imp.iter() }

    type InsertCallbackId = VecF64InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecF64InsertCallbackId {
        VecF64InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: VecF64InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = VecF64DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecF64DeleteCallbackId {
        VecF64DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: VecF64DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {

        let _table = client_cache.get_or_make_table::<VecF64>("vec_f64");
}
#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<VecF64>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .map_err(|e| {
             __sdk::InternalError::failed_parse(
                "TableUpdate<VecF64>",
                "TableUpdate",
            ).with_cause(e).into()
        })
}
