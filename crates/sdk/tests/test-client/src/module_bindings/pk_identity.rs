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
pub struct PkIdentity {
    pub i: Identity,
    pub data: i32,
}

impl TableType for PkIdentity {
    const TABLE_NAME: &'static str = "PkIdentity";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for PkIdentity {
    type PrimaryKey = Identity;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.i
    }
}

impl PkIdentity {
    #[allow(unused)]
    pub fn filter_by_i(i: Identity) -> Option<Self> {
        Self::find(|row| row.i == i)
    }
    #[allow(unused)]
    pub fn filter_by_data(data: i32) -> TableIter<Self> {
        Self::filter(|row| row.data == data)
    }
}
