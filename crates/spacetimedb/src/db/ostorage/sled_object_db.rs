use std::path::Path;
use anyhow::Error;
use bytes::Bytes;
use crate::db::ostorage::ObjectDB;
use crate::hash::{Hash, hash_bytes};
use sled;
use sled::Mode::HighThroughput;

pub struct SledObjectDB {
    db: sled::Db,
}

impl SledObjectDB {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, anyhow::Error> {

        let config = sled::Config::default()
            .path(path)
            .flush_every_ms(Some(50))
            .mode(HighThroughput);
        let db = config.open()?;

        Ok(Self { db })
    }
}

impl ObjectDB for SledObjectDB {

    fn add(&mut self, bytes: Vec<u8>) -> Hash {
        let hash = hash_bytes(&bytes);

        self.db.insert(hash.as_slice(), bytes.as_slice()).unwrap();

        hash
    }

    fn get(&self, hash: Hash) -> Option<Bytes> {
        match self.db.get(hash.as_slice()) {
            Ok(v) => {
                v.map(|v| {
                    let bytes = bytes::Bytes::from(v.to_vec());
                    bytes
                })
            }
            Err(_) => {
                None
            }
        }
    }

    fn flush(&mut self) -> Result<(), Error> {
        match self.db.flush() {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                Err(anyhow::Error::new(e))
            }
        }
    }

    fn sync_all(&mut self) -> Result<(), Error> {
        self.flush()
    }
}

#[cfg(test)]
mod tests {
    use crate::db::ostorage::sled_object_db::SledObjectDB;
    use crate::db::ostorage::ObjectDB;

    use crate::hash::hash_bytes;
    use anyhow::Error;
    use tempdir::TempDir;

    const TEST_DB_DIR_PREFIX: &str = "sledb_test";
    const TEST_DATA1: &[u8; 21] = b"this is a byte string";
    const TEST_DATA2: &[u8; 26] = b"this is also a byte string";

    fn setup() -> Result<SledObjectDB, Error> {
        let tmp_dir = TempDir::new(TEST_DB_DIR_PREFIX).unwrap();
        SledObjectDB::open(tmp_dir.path())
    }

    #[test]
    fn test_add_and_get() {
        let mut db = setup().unwrap();

        let hash1 = db.add(TEST_DATA1.to_vec());
        let hash2 = db.add(TEST_DATA2.to_vec());

        let result = db.get(hash1).unwrap();
        assert_eq!(TEST_DATA1, result.to_vec().as_slice());

        let result = db.get(hash2).unwrap();
        assert_eq!(TEST_DATA2, result.to_vec().as_slice());
    }

    #[test]
    fn test_flush() {
        let mut db = setup().unwrap();

        db.add(TEST_DATA1.to_vec());
        db.add(TEST_DATA2.to_vec());

        assert!(db.flush().is_ok());
    }

    #[test]
    fn test_flush_sync_all() {
        let mut db = setup().unwrap();

        db.add(TEST_DATA1.to_vec());
        db.add(TEST_DATA2.to_vec());

        assert!(db.sync_all().is_ok());
    }

    #[test]
    fn test_miss() {
        let mut db = setup().unwrap();

        let _hash2 = db.add(TEST_DATA2.to_vec());

        let hash = hash_bytes(TEST_DATA1);
        let result = db.get(hash);

        assert!(result.is_none());
    }
}
