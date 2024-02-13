// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize, F32, F64},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct UpdatePkBoolArgs {
    pub b: bool,
    pub data: i32,
}

impl Reducer for UpdatePkBoolArgs {
    const REDUCER_NAME: &'static str = "update_pk_bool";
}

#[allow(unused)]
pub fn update_pk_bool(b: bool, data: i32) {
    UpdatePkBoolArgs { b, data }.invoke();
}

#[allow(unused)]
pub fn on_update_pk_bool(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &bool, &i32) + Send + 'static,
) -> ReducerCallbackId<UpdatePkBoolArgs> {
    UpdatePkBoolArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let UpdatePkBoolArgs { b, data } = __args;
        __callback(__identity, __addr, __status, b, data);
    })
}

#[allow(unused)]
pub fn once_on_update_pk_bool(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &bool, &i32) + Send + 'static,
) -> ReducerCallbackId<UpdatePkBoolArgs> {
    UpdatePkBoolArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let UpdatePkBoolArgs { b, data } = __args;
        __callback(__identity, __addr, __status, b, data);
    })
}

#[allow(unused)]
pub fn remove_on_update_pk_bool(id: ReducerCallbackId<UpdatePkBoolArgs>) {
    UpdatePkBoolArgs::remove_on_reducer(id);
}
