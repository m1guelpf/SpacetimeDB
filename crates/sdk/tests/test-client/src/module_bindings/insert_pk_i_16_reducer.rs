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
pub struct InsertPkI16Args {
    pub n: i16,
    pub data: i32,
}

impl Reducer for InsertPkI16Args {
    const REDUCER_NAME: &'static str = "insert_pk_i16";
}

#[allow(unused)]
pub fn insert_pk_i_16(n: i16, data: i32) {
    InsertPkI16Args { n, data }.invoke();
}

#[allow(unused)]
pub fn on_insert_pk_i_16(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &i16, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertPkI16Args> {
    InsertPkI16Args::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertPkI16Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn once_on_insert_pk_i_16(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &i16, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertPkI16Args> {
    InsertPkI16Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertPkI16Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn remove_on_insert_pk_i_16(id: ReducerCallbackId<InsertPkI16Args>) {
    InsertPkI16Args::remove_on_reducer(id);
}
