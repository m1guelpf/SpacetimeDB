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
pub struct InsertUniqueI16Args {
    pub n: i16,
    pub data: i32,
}

impl Reducer for InsertUniqueI16Args {
    const REDUCER_NAME: &'static str = "insert_unique_i16";
}

#[allow(unused)]
pub fn insert_unique_i_16(n: i16, data: i32) {
    InsertUniqueI16Args { n, data }.invoke();
}

#[allow(unused)]
pub fn on_insert_unique_i_16(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &i16, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertUniqueI16Args> {
    InsertUniqueI16Args::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertUniqueI16Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn once_on_insert_unique_i_16(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &i16, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertUniqueI16Args> {
    InsertUniqueI16Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertUniqueI16Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn remove_on_insert_unique_i_16(id: ReducerCallbackId<InsertUniqueI16Args>) {
    InsertUniqueI16Args::remove_on_reducer(id);
}
