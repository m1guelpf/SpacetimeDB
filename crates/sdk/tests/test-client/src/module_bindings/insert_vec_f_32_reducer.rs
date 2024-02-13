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
pub struct InsertVecF32Args {
    pub f: Vec<F32>,
}

impl Reducer for InsertVecF32Args {
    const REDUCER_NAME: &'static str = "insert_vec_f32";
}

#[allow(unused)]
pub fn insert_vec_f_32(f: Vec<F32>) {
    InsertVecF32Args { f }.invoke();
}

#[allow(unused)]
pub fn on_insert_vec_f_32(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Vec<F32>) + Send + 'static,
) -> ReducerCallbackId<InsertVecF32Args> {
    InsertVecF32Args::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecF32Args { f } = __args;
        __callback(__identity, __addr, __status, f);
    })
}

#[allow(unused)]
pub fn once_on_insert_vec_f_32(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Vec<F32>) + Send + 'static,
) -> ReducerCallbackId<InsertVecF32Args> {
    InsertVecF32Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecF32Args { f } = __args;
        __callback(__identity, __addr, __status, f);
    })
}

#[allow(unused)]
pub fn remove_on_insert_vec_f_32(id: ReducerCallbackId<InsertVecF32Args>) {
    InsertVecF32Args::remove_on_reducer(id);
}
