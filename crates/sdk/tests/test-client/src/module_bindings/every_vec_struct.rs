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
pub struct EveryVecStruct {
    pub a: Vec<u8>,
    pub b: Vec<u16>,
    pub c: Vec<u32>,
    pub d: Vec<u64>,
    pub e: Vec<u128>,
    pub f: Vec<i8>,
    pub g: Vec<i16>,
    pub h: Vec<i32>,
    pub i: Vec<i64>,
    pub j: Vec<i128>,
    pub k: Vec<bool>,
    pub l: Vec<F32>,
    pub m: Vec<F64>,
    pub n: Vec<String>,
    pub o: Vec<Identity>,
    pub p: Vec<Address>,
}
