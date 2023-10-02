// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

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
pub struct UniqueBool {
    pub b: bool,
    pub data: i32,
}

impl TableType for UniqueBool {
    const TABLE_NAME: &'static str = "UniqueBool";
    type ReducerEvent = super::ReducerEvent;
}

impl UniqueBool {
    #[allow(unused)]
    pub fn filter_by_b(b: bool) -> Option<Self> {
        Self::find(|row| row.b == b)
    }
    #[allow(unused)]
    pub fn filter_by_data(data: i32) -> TableIter<Self> {
        Self::filter(|row| row.data == data)
    }
}
