use std::fs;
use std::path::Path;
use anyhow::Error;
use bytes::Bytes;
use crate::db::ostorage::ObjectDB;
use rocksdb::{ColumnFamilyDescriptor, DB, Options};
use crate::hash::{Hash, hash_bytes};

pub struct RocksDBObjectDB {
    db: DB,
}

impl RocksDBObjectDB {

    const OBJECTS_CF : &'static str = "objects";

    pub fn open(path: impl AsRef<Path>) -> Result<Self, anyhow::Error> {
        let root = path.as_ref();
        fs::create_dir_all(root).unwrap();

        // Create the column family for our object data.
        // We need at least one column family or Rocks doesn't seem to actually properly keep files
        // on flush, etc.
        let cf = ColumnFamilyDescriptor::new(RocksDBObjectDB::OBJECTS_CF, Options::default());

        let mut db_opts = rocksdb::Options::default();
        db_opts.create_if_missing(true);
        db_opts.create_missing_column_families(true);
        db_opts.set_use_fsync(true); // Make things a bit more durable in theory.

        let db = DB::open_cf_descriptors(&db_opts, root, vec![cf])?;

        Ok(RocksDBObjectDB { db })
    }
}

impl ObjectDB for RocksDBObjectDB {
    fn add(&mut self, bytes: Vec<u8>) -> Hash {
        let hash = hash_bytes(&bytes);

        let cf = self.db.cf_handle(RocksDBObjectDB::OBJECTS_CF).unwrap();

        self.db.put_cf(&cf, hash.as_slice(), bytes.as_slice()).unwrap();

        hash
    }

    fn get(&self, hash: Hash) -> Option<Bytes> {
        let cf = self.db.cf_handle(RocksDBObjectDB::OBJECTS_CF).unwrap();

        match self.db.get_cf(cf,hash.as_slice()) {
           Ok(Some(value)) => {
               Some(bytes::Bytes::from(value))
           }
           Ok(None) => {
               None
           }
           Err(e) => {
                panic!("error in rocksdb::get: {:?}", e)
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
    use crate::hash::hash_bytes;
    use anyhow::Error;
    use tempdir::TempDir;
    use crate::db::ostorage::ObjectDB;
    use crate::db::ostorage::rocks_object_db::RocksDBObjectDB;

    const TEST_DB_DIR_PREFIX: &str = "rocksdb_test";
    const TEST_DATA1: &[u8; 21] = b"this is a byte string";
    const TEST_DATA2: &[u8; 26] = b"this is also a byte string";

    fn setup() -> Result<RocksDBObjectDB, Error> {
        let tmp_dir = TempDir::new(TEST_DB_DIR_PREFIX).unwrap();
        RocksDBObjectDB::open(tmp_dir.path())
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
