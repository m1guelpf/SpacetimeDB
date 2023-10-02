// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use super::simple_enum::SimpleEnum;
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
pub struct InsertOneSimpleEnumArgs {
    pub e: SimpleEnum,
}

impl Reducer for InsertOneSimpleEnumArgs {
    const REDUCER_NAME: &'static str = "insert_one_simple_enum";
}

#[allow(unused)]
pub fn insert_one_simple_enum(e: SimpleEnum) {
    InsertOneSimpleEnumArgs { e }.invoke();
}

#[allow(unused)]
pub fn on_insert_one_simple_enum(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &SimpleEnum) + Send + 'static,
) -> ReducerCallbackId<InsertOneSimpleEnumArgs> {
    InsertOneSimpleEnumArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertOneSimpleEnumArgs { e } = __args;
        __callback(__identity, __addr, __status, e);
    })
}

#[allow(unused)]
pub fn once_on_insert_one_simple_enum(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &SimpleEnum) + Send + 'static,
) -> ReducerCallbackId<InsertOneSimpleEnumArgs> {
    InsertOneSimpleEnumArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertOneSimpleEnumArgs { e } = __args;
        __callback(__identity, __addr, __status, e);
    })
}

#[allow(unused)]
pub fn remove_on_insert_one_simple_enum(id: ReducerCallbackId<InsertOneSimpleEnumArgs>) {
    InsertOneSimpleEnumArgs::remove_on_reducer(id);
}
