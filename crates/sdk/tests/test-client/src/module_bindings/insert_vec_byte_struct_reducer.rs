// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use super::byte_struct::ByteStruct;
#[allow(unused)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InsertVecByteStructArgs {
    pub s: Vec<ByteStruct>,
}

impl Reducer for InsertVecByteStructArgs {
    const REDUCER_NAME: &'static str = "insert_vec_byte_struct";
}

#[allow(unused)]
pub fn insert_vec_byte_struct(s: Vec<ByteStruct>) {
    InsertVecByteStructArgs { s }.invoke();
}

#[allow(unused)]
pub fn on_insert_vec_byte_struct(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Vec<ByteStruct>) + Send + 'static,
) -> ReducerCallbackId<InsertVecByteStructArgs> {
    InsertVecByteStructArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecByteStructArgs { s } = __args;
        __callback(__identity, __addr, __status, s);
    })
}

#[allow(unused)]
pub fn once_on_insert_vec_byte_struct(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Vec<ByteStruct>) + Send + 'static,
) -> ReducerCallbackId<InsertVecByteStructArgs> {
    InsertVecByteStructArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecByteStructArgs { s } = __args;
        __callback(__identity, __addr, __status, s);
    })
}

#[allow(unused)]
pub fn remove_on_insert_vec_byte_struct(id: ReducerCallbackId<InsertVecByteStructArgs>) {
    InsertVecByteStructArgs::remove_on_reducer(id);
}
