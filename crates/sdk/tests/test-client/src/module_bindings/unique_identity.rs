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
pub struct UniqueIdentity {
    pub i: Identity,
    pub data: i32,
}

impl TableType for UniqueIdentity {
    const TABLE_NAME: &'static str = "UniqueIdentity";
    type ReducerEvent = super::ReducerEvent;
}

impl UniqueIdentity {
    #[allow(unused)]
    pub fn filter_by_i(i: Identity) -> Option<Self> {
        Self::find(|row| row.i == i)
    }
    #[allow(unused)]
    pub fn filter_by_data(data: i32) -> TableIter<Self> {
        Self::filter(|row| row.data == data)
    }
}
