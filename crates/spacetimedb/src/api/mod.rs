use crate::{
    db::object_db::ObjectDB,
    hash::Hash,
    identity::{alloc_spacetime_identity, decode_token, encode_token},
    postgres,
};
use lazy_static::lazy_static;
use std::sync::Mutex;
// use rocksdb;

/*
-x SpacetimeDB API
-x indexing of columns
-x atomic transactions (including creation of tables)
-x diff commits
- snapshot commits
-x SQL query -> API call support
-x Smart contract which calls APIs
- Schema migration
- Schema code generation to improve ergonomics of smart contract
- Metrics tables (data size, tx/sec)
- Dashboard for displaying metrics
-x (some way to upload smart contracts and track the versions to our website??)
-x subscription API (subscription queries)
- read permissions
- partial in-memory state
- (client library for syncing data based on queries??)
-x non-primitive type columns (e.g. struct in column)
*/

lazy_static! {
    pub static ref MODULE_ODB: Mutex<ObjectDB> = Mutex::new(ObjectDB::open("/stdb/module_odb").unwrap());
}

pub async fn spacetime_identity() -> Result<(Hash, String), anyhow::Error> {
    let identity = alloc_spacetime_identity().await?;
    let identity_token = encode_token(identity)?;
    Ok((identity, identity_token))
}

pub async fn spacetime_identity_associate_email(email: &str, identity_token: &str) -> Result<(), anyhow::Error> {
    let token = decode_token(identity_token)?;
    let hex_identity = token.claims.hex_identity;
    let client = postgres::get_client().await;
    client
        .query(
            "INSERT INTO registry.email (st_identity, email) VALUES ($1, $2)",
            &[&hex_identity, &email],
        )
        .await?;
    Ok(())
}

pub mod database {
    use crate::{
        hash::Hash,
        logs::{self, init_log},
        postgres,
        wasm_host::{self, get_host},
    };

    use super::MODULE_ODB;

    // TODO: Verify identity?
    pub async fn init_module(
        hex_identity: &str,
        name: &str,
        force: bool,
        wasm_bytes: Vec<u8>,
    ) -> Result<Hash, anyhow::Error> {
        let client = postgres::get_client().await;
        let result = client
            .query(
                "SELECT * from registry.module WHERE actor_name = $1 AND st_identity = $2",
                &[&name, &hex_identity],
            )
            .await;

        let identity = *Hash::from_slice(&hex::decode(hex_identity).unwrap());
        let host = wasm_host::get_host();

        if let Ok(rows) = result {
            if rows.len() > 0 && !force {
                return Err(anyhow::anyhow!("Cannot init existing actor."));
            } else {
                host.delete_module(identity, name.into()).await?;
            }
        }

        let address = host.init_module(identity, name.into(), wasm_bytes.clone()).await?;

        // If the module successfully initialized add it to the object database
        {
            let mut object_db = MODULE_ODB.lock().unwrap();
            object_db.add(wasm_bytes);
        }

        // Store this module metadata in postgres
        client.query(
            "INSERT INTO registry.module (actor_name, st_identity, module_version, module_address) VALUES ($1, $2, $3, $4)", 
            &[&name, &hex_identity, &0_i32, &hex::encode(address)]
        ).await?;

        init_log(identity, name);

        Ok(address)
    }

    pub async fn update_module(hex_identity: &str, name: &str, wasm_bytes: Vec<u8>) -> Result<Hash, anyhow::Error> {
        let client = postgres::get_client().await;
        let identity = *Hash::from_slice(&hex::decode(hex_identity).unwrap());
        let host = wasm_host::get_host();
        let address = host.update_module(identity, name.into(), wasm_bytes.clone()).await?;

        // If the module successfully initialized add it to the object database
        {
            let mut object_db = MODULE_ODB.lock().unwrap();
            object_db.add(wasm_bytes);
        }

        let result = client
            .query(
                "UPDATE registry.module SET module_address = $1, module_version = module_version + 1 WHERE actor_name = $2 AND st_identity = $3",
                &[&hex::encode(address), &name, &hex_identity],
            )
            .await;

        if let Err(err) = result {
            return Err(anyhow::anyhow!("Error updating module. {}", err));
        }

        init_log(identity, name);
        Ok(address)
    }

    pub async fn call(
        identity: &str,
        name: &str,
        caller_identity: Hash,
        reducer: String,
        arg_bytes: Vec<u8>,
    ) -> Result<(), anyhow::Error> {
        // TODO: optimize by loading all these into memory
        let client = postgres::get_client().await;
        let result = client
            .query_one(
                r"
            SELECT DISTINCT ON (actor_name, st_identity, module_version)
                actor_name, st_identity, module_version, module_address
            FROM registry.module
                WHERE actor_name = $1
                AND st_identity = $2
            ORDER BY module_version DESC;",
                &[&name, &identity],
            )
            .await;
        let _ = result?;
        let identity_hash: Hash = Hash::from_iter(hex::decode(identity).unwrap());

        get_host()
            .call_reducer(identity_hash, name.to_string(), caller_identity, reducer, arg_bytes)
            .await?;

        Ok(())
    }

    pub fn query(_identity: String, _name: String, _query: String) {
        unimplemented!()
    }

    pub async fn logs(hex_identity: &str, name: &str, num_lines: u32) -> String {
        let identity = *Hash::from_slice(&hex::decode(hex_identity).unwrap());
        logs::read_latest(identity, name, num_lines).await
    }

    // Optional
    pub fn revert_ts(_identity: String, _name: String, _timestamp: u64) {
        unimplemented!()
    }

    pub fn revert_hash(_identity: String, _name: String, _hash: Hash) {
        unimplemented!()
    }

    pub fn address(_identity: String, _name: String) -> String {
        unimplemented!()
    }

    pub fn metrics(_identity: String, _name: String) {
        unimplemented!()
    }

    pub mod energy {
        pub fn info(_identity: String, _name: String) {
            unimplemented!()
        }

        pub fn buy(_identity: String, _name: String, _amount: u64) {
            unimplemented!()
        }
    }
}
