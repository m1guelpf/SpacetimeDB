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
pub struct InsertPkI8Args {
    pub n: i8,
    pub data: i32,
}

impl Reducer for InsertPkI8Args {
    const REDUCER_NAME: &'static str = "insert_pk_i8";
}

#[allow(unused)]
pub fn insert_pk_i_8(n: i8, data: i32) {
    InsertPkI8Args { n, data }.invoke();
}

#[allow(unused)]
pub fn on_insert_pk_i_8(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &i8, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertPkI8Args> {
    InsertPkI8Args::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertPkI8Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn once_on_insert_pk_i_8(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &i8, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertPkI8Args> {
    InsertPkI8Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertPkI8Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn remove_on_insert_pk_i_8(id: ReducerCallbackId<InsertPkI8Args>) {
    InsertPkI8Args::remove_on_reducer(id);
}
