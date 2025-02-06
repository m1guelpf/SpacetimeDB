// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{
	self as __sdk,
	__lib,
	__sats,
	__ws,
};
use super::vec_identity_type::VecIdentity;

/// Table handle for the table `vec_identity`.
///
/// Obtain a handle from the [`VecIdentityTableAccess::vec_identity`] method on [`super::RemoteTables`],
/// like `ctx.db.vec_identity()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.vec_identity().on_insert(...)`.
pub struct VecIdentityTableHandle<'ctx> {
    imp: __sdk::TableHandle<VecIdentity>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `vec_identity`.
///
/// Implemented for [`super::RemoteTables`].
pub trait VecIdentityTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`VecIdentityTableHandle`], which mediates access to the table `vec_identity`.
    fn vec_identity(&self) -> VecIdentityTableHandle<'_>;
}

impl VecIdentityTableAccess for super::RemoteTables {
    fn vec_identity(&self) -> VecIdentityTableHandle<'_> {
        VecIdentityTableHandle {
            imp: self.imp.get_table::<VecIdentity>("vec_identity"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct VecIdentityInsertCallbackId(__sdk::CallbackId);
pub struct VecIdentityDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for VecIdentityTableHandle<'ctx> {
    type Row = VecIdentity;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 { self.imp.count() }
    fn iter(&self) -> impl Iterator<Item = VecIdentity> + '_ { self.imp.iter() }

    type InsertCallbackId = VecIdentityInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecIdentityInsertCallbackId {
        VecIdentityInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: VecIdentityInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = VecIdentityDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecIdentityDeleteCallbackId {
        VecIdentityDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: VecIdentityDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {

        let _table = client_cache.get_or_make_table::<VecIdentity>("vec_identity");
}
#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<VecIdentity>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .map_err(|e| {
             __sdk::InternalError::failed_parse(
                "TableUpdate<VecIdentity>",
                "TableUpdate",
            ).with_cause(e).into()
        })
}
