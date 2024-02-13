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
pub struct Message {
    pub sender: Identity,
    pub sent: u64,
    pub text: String,
}

impl TableType for Message {
    const TABLE_NAME: &'static str = "Message";
    type ReducerEvent = super::ReducerEvent;
}

impl Message {
    #[allow(unused)]
    pub fn filter_by_sender(sender: Identity) -> TableIter<Self> {
        Self::filter(|row| row.sender == sender)
    }
    #[allow(unused)]
    pub fn filter_by_sent(sent: u64) -> TableIter<Self> {
        Self::filter(|row| row.sent == sent)
    }
    #[allow(unused)]
    pub fn filter_by_text(text: String) -> TableIter<Self> {
        Self::filter(|row| row.text == text)
    }
}
