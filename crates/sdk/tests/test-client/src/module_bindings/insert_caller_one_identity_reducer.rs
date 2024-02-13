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
pub struct InsertCallerOneIdentityArgs {}

impl Reducer for InsertCallerOneIdentityArgs {
    const REDUCER_NAME: &'static str = "insert_caller_one_identity";
}

#[allow(unused)]
pub fn insert_caller_one_identity() {
    InsertCallerOneIdentityArgs {}.invoke();
}

#[allow(unused)]
pub fn on_insert_caller_one_identity(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<InsertCallerOneIdentityArgs> {
    InsertCallerOneIdentityArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertCallerOneIdentityArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn once_on_insert_caller_one_identity(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<InsertCallerOneIdentityArgs> {
    InsertCallerOneIdentityArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertCallerOneIdentityArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn remove_on_insert_caller_one_identity(id: ReducerCallbackId<InsertCallerOneIdentityArgs>) {
    InsertCallerOneIdentityArgs::remove_on_reducer(id);
}
