// Copyright (c) 2021-2022 Yuki Kishimoto
// Distributed under the MIT software license

#![allow(dead_code)]

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

pub use rocksdb::{
    BoundColumnFamily, ColumnFamilyDescriptor, DBCompactionStyle, DBCompressionType, Direction,
    IteratorMode, WriteBatch, WriteOptions,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Store {
    db: Arc<rocksdb::DB>,
    column_families: Vec<String>,
}

pub enum IteratorOptions<'a> {
    WithMode(IteratorMode<'a>),
    WitLimit(usize, bool),
}

impl<'a> Default for IteratorOptions<'a> {
    fn default() -> Self {
        Self::WithMode(IteratorMode::Start)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("RocksDB error: {0}")]
    RocksDb(#[from] rocksdb::Error),
    #[error("Impossible to put")]
    FailedToPut,
    #[error("Impossible to get")]
    FailedToGet,
    #[error("Impossible to delete")]
    FailedToDelete,
    #[error("Impossible to deserialize")]
    FailedToDeserialize,
    #[error("Impossible to serialize")]
    FailedToSerialize,
    #[error("Value not found")]
    ValueNotFound,
}

pub fn vec_to_vec_string<I, T>(iter: I) -> Vec<String>
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    iter.into_iter().map(Into::into).collect()
}

fn default_opts() -> rocksdb::Options {
    let mut opts = rocksdb::Options::default();
    opts.set_keep_log_file_num(10);
    opts.set_max_open_files(100);
    opts.set_compaction_style(DBCompactionStyle::Level);
    opts.set_compression_type(DBCompressionType::Zstd);
    opts.set_target_file_size_base(256 << 20);
    opts.set_write_buffer_size(256 << 20);
    opts.set_enable_write_thread_adaptive_yield(true);
    opts.set_disable_auto_compactions(true); // for initial bulk load
    opts
}

impl Store {
    pub fn open<P>(path: P, column_families: &[&str]) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        log::debug!("Opening {}", path.display());

        let mut db_opts = default_opts();
        db_opts.create_if_missing(true);
        db_opts.create_missing_column_families(true);

        let db = match rocksdb::DB::open_cf_descriptors(
            &db_opts,
            path,
            Self::create_cf_descriptors(column_families),
        ) {
            Ok(data) => data,
            Err(error) => panic!("{:?}", error),
        };

        match db.live_files() {
            Ok(live_files) => log::info!(
                "{}: {} SST files, {} GB, {} Grows",
                path.display(),
                live_files.len(),
                live_files.iter().map(|f| f.size).sum::<usize>() as f64 / 1e9,
                live_files.iter().map(|f| f.num_entries).sum::<u64>() as f64 / 1e9
            ),
            Err(_) => log::warn!("Impossible to get live files"),
        };

        Ok(Self {
            db: Arc::new(db),
            column_families: vec_to_vec_string(column_families.to_vec()),
        })
    }

    fn create_cf_descriptors(column_families: &[&str]) -> Vec<ColumnFamilyDescriptor> {
        column_families
            .iter()
            .map(|&name| ColumnFamilyDescriptor::new(name, default_opts()))
            .collect()
    }

    pub fn cf_handle(&self, name: &str) -> Arc<BoundColumnFamily> {
        self.db
            .cf_handle(name)
            .unwrap_or_else(|| panic!("missing {}_CF", name.to_uppercase()))
    }

    pub fn serialize<T>(&self, data: T) -> Result<Vec<u8>, Error>
    where
        T: Serialize + std::fmt::Debug,
    {
        match serde_json::to_string(&data) {
            Ok(serialized) => Ok(serialized.into_bytes()),
            Err(_) => Err(Error::FailedToSerialize),
        }
    }

    pub fn deserialize<T>(&self, data: Vec<u8>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        match serde_json::from_slice::<T>(&data) {
            Ok(u) => Ok(u),
            Err(_) => Err(Error::FailedToDeserialize),
        }
    }

    pub fn get<K>(&self, cf: Arc<BoundColumnFamily>, key: K) -> Result<Vec<u8>, Error>
    where
        K: AsRef<[u8]>,
    {
        match self.db.get_pinned_cf(&cf, key) {
            Ok(Some(value)) => Ok(value.to_vec()),
            Ok(None) => Err(Error::ValueNotFound),
            Err(_) => Err(Error::FailedToGet),
        }
    }

    pub fn get_deserialized<K, V>(&self, cf: Arc<BoundColumnFamily>, key: K) -> Result<V, Error>
    where
        K: AsRef<[u8]>,
        V: DeserializeOwned,
    {
        self.deserialize::<V>(self.get(cf, key)?)
    }

    pub fn put<K, V>(&self, cf: Arc<BoundColumnFamily>, key: K, value: V) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        match self.db.put_cf(&cf, key, value) {
            Ok(_) => Ok(()),
            Err(error) => {
                log::error!("Impossible to put value in database: {}", error);
                Err(Error::FailedToPut)
            }
        }
    }

    pub fn put_serialized<K, V>(
        &self,
        cf: Arc<BoundColumnFamily>,
        key: K,
        value: &V,
    ) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
        V: Serialize + std::fmt::Debug,
    {
        self.put(cf, key, self.serialize(value)?)
    }

    pub fn iterator_with_mode(
        &self,
        cf: Arc<BoundColumnFamily>,
        mode: IteratorMode,
    ) -> Result<HashMap<Vec<u8>, Vec<u8>>, Error> {
        let mut collection: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();

        let iter = self.db.iterator_cf(&cf, mode);
        for item in iter {
            let (key, value) = item?;
            collection.insert(key.to_vec(), value.to_vec());
        }
        Ok(collection)
    }

    pub fn iterator_with_limit(
        &self,
        cf: Arc<BoundColumnFamily>,
        limit: usize,
        ascending: bool,
    ) -> Result<HashMap<Vec<u8>, Vec<u8>>, Error> {
        let mut collection = if ascending {
            self.iterator_with_mode(cf, IteratorMode::Start)?
        } else {
            self.iterator_with_mode(cf, IteratorMode::End)?
        };
        Ok(collection.drain().take(limit).collect())
    }

    pub fn iterator_with_opt(
        &self,
        cf: Arc<BoundColumnFamily>,
        options: IteratorOptions,
    ) -> Result<HashMap<Vec<u8>, Vec<u8>>, Error> {
        match options {
            IteratorOptions::WithMode(mode) => self.iterator_with_mode(cf, mode),
            IteratorOptions::WitLimit(limit, ascending) => {
                self.iterator_with_limit(cf, limit, ascending)
            }
        }
    }

    pub fn iterator(&self, cf: Arc<BoundColumnFamily>) -> Result<HashMap<Vec<u8>, Vec<u8>>, Error> {
        self.iterator_with_opt(cf, IteratorOptions::default())
    }

    pub fn iterator_serialized_with_opt<K, V>(
        &self,
        cf: Arc<BoundColumnFamily>,
        options: IteratorOptions,
    ) -> Result<HashMap<K, V>, Error>
    where
        K: DeserializeOwned + std::cmp::Eq + std::hash::Hash,
        V: DeserializeOwned,
    {
        let mut collection = HashMap::new();

        for (key_bytes, value_bytes) in self.iterator_with_opt(cf.clone(), options)?.into_iter() {
            match self.deserialize::<K>(key_bytes.to_vec()) {
                Ok(key) => {
                    match self.deserialize::<V>(value_bytes.to_vec()) {
                        Ok(value) => {
                            collection.insert(key, value);
                        }
                        Err(error) => {
                            log::error!("Failed to deserialize value: {:?}", error);
                            let _ = self.delete(cf.clone(), key_bytes);
                        }
                    };
                }
                Err(_) => log::error!("Failed to deserialize key"),
            };
        }

        Ok(collection)
    }

    pub fn iterator_serialized<K, V>(
        &self,
        cf: Arc<BoundColumnFamily>,
    ) -> Result<HashMap<K, V>, Error>
    where
        K: DeserializeOwned + std::cmp::Eq + std::hash::Hash,
        V: DeserializeOwned,
    {
        self.iterator_serialized_with_opt(cf, IteratorOptions::default())
    }

    pub fn iterator_key_serialized_with_opt<T>(
        &self,
        cf: Arc<BoundColumnFamily>,
        options: IteratorOptions,
    ) -> Result<Vec<T>, Error>
    where
        T: DeserializeOwned,
    {
        let mut collection: Vec<T> = Vec::new();

        for key in self.iterator_with_opt(cf.clone(), options)?.into_keys() {
            match self.deserialize::<T>(key.to_vec()) {
                Ok(key) => collection.push(key),
                Err(_) => log::error!("Failed to deserialize key"),
            };
        }

        Ok(collection)
    }

    pub fn iterator_key_serialized<T>(&self, cf: Arc<BoundColumnFamily>) -> Result<Vec<T>, Error>
    where
        T: DeserializeOwned,
    {
        self.iterator_key_serialized_with_opt(cf, IteratorOptions::default())
    }

    pub fn iterator_value_serialized_with_opt<T>(
        &self,
        cf: Arc<BoundColumnFamily>,
        options: IteratorOptions,
    ) -> Result<Vec<T>, Error>
    where
        T: DeserializeOwned,
    {
        let mut collection: Vec<T> = Vec::new();

        for value in self.iterator_with_opt(cf.clone(), options)?.into_values() {
            match self.deserialize::<T>(value.to_vec()) {
                Ok(value) => collection.push(value),
                Err(error) => log::error!("Failed to deserialize value: {:?}", error),
            };
        }

        Ok(collection)
    }

    pub fn iterator_value_serialized<T>(&self, cf: Arc<BoundColumnFamily>) -> Result<Vec<T>, Error>
    where
        T: DeserializeOwned,
    {
        self.iterator_value_serialized_with_opt(cf, IteratorOptions::default())
    }

    pub fn delete<K>(&self, cf: Arc<BoundColumnFamily>, key: K) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
    {
        match self.db.delete_cf(&cf, key) {
            Ok(_) => Ok(()),
            Err(error) => {
                log::error!("Impossible to delete key from database: {}", error);
                Err(Error::FailedToDelete)
            }
        }
    }

    pub fn write(&self, batch: WriteBatch) -> Result<(), Error> {
        self.db.write(batch)?;
        Ok(())
    }

    pub fn flush(&self) {
        self.column_families.iter().for_each(|name| {
            let cf = self
                .db
                .cf_handle(name.as_str())
                .unwrap_or_else(|| panic!("missing {}_CF", name.to_uppercase()));
            match self.db.flush_cf(&cf) {
                Ok(_) => log::debug!("{} cf flushed", name),
                Err(error) => log::error!("Impossible to flush {} cf: {}", name, error),
            };
        });

        self.start_compactions();
    }

    fn start_compactions(&self) {
        self.column_families.iter().for_each(|name| {
            log::debug!("starting {} compaction", name);
            let cf = self
                .db
                .cf_handle(name.as_str())
                .unwrap_or_else(|| panic!("missing {}_CF", name.to_uppercase()));
            self.db.compact_range_cf(&cf, None::<&[u8]>, None::<&[u8]>);
        });

        log::debug!("finished full compaction");

        self.column_families.iter().for_each(|name| {
            let cf = self
                .db
                .cf_handle(name.as_str())
                .unwrap_or_else(|| panic!("missing {}_CF", name.to_uppercase()));
            self.db
                .set_options_cf(&cf, &[("disable_auto_compactions", "false")])
                .expect("failed to start auto-compactions");
        });
        log::debug!("auto-compactions enabled");
    }
}

impl Drop for Store {
    fn drop(&mut self) {
        log::trace!("Closing Database");
    }
}

pub trait WriteSerializedBatch {
    fn put_serialized<K, V>(
        &mut self,
        cf: Arc<BoundColumnFamily>,
        key: K,
        value: &V,
    ) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
        V: Serialize + std::fmt::Debug;
}

impl WriteSerializedBatch for WriteBatch {
    fn put_serialized<K, V>(
        &mut self,
        cf: Arc<BoundColumnFamily>,
        key: K,
        value: &V,
    ) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
        V: Serialize + std::fmt::Debug,
    {
        match serde_json::to_string(&value) {
            Ok(serialized) => {
                self.put_cf(&cf, key, serialized.into_bytes());
                Ok(())
            }
            Err(_) => Err(Error::FailedToSerialize),
        }
    }
}
