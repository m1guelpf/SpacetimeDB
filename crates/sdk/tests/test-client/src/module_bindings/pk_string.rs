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
pub struct PkString {
    pub s: String,
    pub data: i32,
}

impl TableType for PkString {
    const TABLE_NAME: &'static str = "PkString";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for PkString {
    type PrimaryKey = String;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.s
    }
}

impl PkString {
    #[allow(unused)]
    pub fn filter_by_s(s: String) -> Option<Self> {
        Self::find(|row| row.s == s)
    }
    #[allow(unused)]
    pub fn filter_by_data(data: i32) -> TableIter<Self> {
        Self::filter(|row| row.data == data)
    }
}
