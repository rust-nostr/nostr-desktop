// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::path::Path;
use std::sync::Arc;

use nostr_sdk::nostr::secp256k1::XOnlyPublicKey;
use nostr_sdk::nostr::{Contact, Event, Kind, KindBase, Sha256Hash};
use nostr_sdk::Result;

pub mod model;
mod rocksdb;
mod util;

use self::model::Profile;
use self::rocksdb::{
    BoundColumnFamily, IteratorMode, Store as RocksStore, WriteBatch, WriteSerializedBatch,
};
use self::util::HashPrefix;

#[derive(Debug, Clone)]
pub struct Store {
    db: RocksStore,
}

// Main CFs
const EVENT_CF: &str = "event";
const AUTHOR_CF: &str = "author";
const CONTACT_CF: &str = "contact";
const PROFILE_CF: &str = "profile";

// Index CFs
const TEXTNOTE_BY_TIMESTAMP: &str = "textnotebytimestamp";

const COLUMN_FAMILIES: &[&str] = &[
    EVENT_CF,
    AUTHOR_CF,
    CONTACT_CF,
    PROFILE_CF,
    TEXTNOTE_BY_TIMESTAMP,
];

impl Store {
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(Self {
            db: RocksStore::open(path, COLUMN_FAMILIES)?,
        })
    }

    fn event_cf(&self) -> Arc<BoundColumnFamily> {
        self.db.cf_handle(EVENT_CF)
    }

    fn author_cf(&self) -> Arc<BoundColumnFamily> {
        self.db.cf_handle(AUTHOR_CF)
    }

    fn profile_cf(&self) -> Arc<BoundColumnFamily> {
        self.db.cf_handle(PROFILE_CF)
    }

    fn contact_cf(&self) -> Arc<BoundColumnFamily> {
        self.db.cf_handle(CONTACT_CF)
    }

    fn textnote_by_timestamp(&self) -> Arc<BoundColumnFamily> {
        self.db.cf_handle(TEXTNOTE_BY_TIMESTAMP)
    }

    pub fn get_event(&self, event_id: Sha256Hash) -> Result<Event> {
        Ok(self
            .db
            .get_deserialized(self.event_cf(), util::hash_prefix(event_id))?)
    }

    pub fn save_event(&self, event: &Event) -> Result<()> {
        let mut batch = WriteBatch::default();

        let event_id_prefix: HashPrefix = util::hash_prefix(event.id);

        match event.kind {
            Kind::Base(KindBase::TextNote) => self.index_by_timestamp(
                &mut batch,
                self.textnote_by_timestamp(),
                event_id_prefix,
                event.created_at,
            )?,
            _ => (),
        };

        if !self.db.key_may_exist(self.event_cf(), event_id_prefix) {
            batch.put_serialized(self.event_cf(), event_id_prefix, event)?;
        }

        Ok(self.db.write(batch)?)
    }

    fn index_by_timestamp(
        &self,
        batch: &mut WriteBatch,
        cf: Arc<BoundColumnFamily>,
        event_id_prefix: HashPrefix,
        timestamp: u64,
    ) -> Result<()> {
        let timestamp = timestamp.to_be_bytes();
        if let Ok(prev_ids) = self
            .db
            .get_deserialized::<HashPrefix, Vec<Vec<u8>>>(cf.clone(), timestamp)
        {
            let event_id_prefix = event_id_prefix.to_vec();
            if !prev_ids.contains(&event_id_prefix) {
                batch.put_serialized(cf, timestamp, &[prev_ids, vec![event_id_prefix]].concat())?;
            }
        } else {
            batch.put_serialized(cf, timestamp, &[event_id_prefix])?;
        };

        Ok(())
    }

    pub fn set_profile(&self, public_key: XOnlyPublicKey, profile: Profile) -> Result<()> {
        Ok(self.db.put(
            self.profile_cf(),
            self.db.serialize(public_key)?,
            self.db.serialize(profile)?,
        )?)
    }

    pub fn get_profile(&self, public_key: XOnlyPublicKey) -> Result<Profile> {
        Ok(self
            .db
            .get_deserialized(self.profile_cf(), self.db.serialize(public_key)?)?)
    }

    pub fn set_contacts(&self, list: Vec<Contact>) -> Result<()> {
        let mut batch = WriteBatch::default();

        for contact in list.iter() {
            batch.put_serialized(self.contact_cf(), self.db.serialize(contact.pk)?, contact)?;
        }

        Ok(self.db.write(batch)?)
    }

    pub fn get_contacts(&self) -> Vec<Contact> {
        let mut contacts: Vec<Contact> = self
            .db
            .iterator_value_serialized_with_mode(self.contact_cf(), IteratorMode::Start)
            .collect();
        contacts.sort();
        contacts
    }

    pub fn set_author(&self, public_key: XOnlyPublicKey) -> Result<()> {
        Ok(self
            .db
            .put(self.author_cf(), self.db.serialize(public_key)?, b"")?)
    }

    pub fn set_authors(&self, authors: Vec<XOnlyPublicKey>) -> Result<()> {
        let mut batch = WriteBatch::default();

        for author in authors.iter() {
            batch.put_cf(&self.author_cf(), self.db.serialize(author)?, b"");
        }

        Ok(self.db.write(batch)?)
    }

    pub fn get_authors(&self) -> Result<Vec<XOnlyPublicKey>> {
        Ok(self.db.iterator_key_serialized(self.author_cf())?)
    }

    pub fn get_feed_ids(&self, limit: usize, page: usize) -> Vec<Vec<u8>> {
        let res: Vec<Vec<Vec<u8>>> = self
            .db
            .iterator_value_serialized_with_mode(self.textnote_by_timestamp(), IteratorMode::End)
            .skip(limit * page)
            .take(limit)
            .collect();
        res.concat()
    }

    pub fn get_feed_by_ids<K, I>(&self, ids: I) -> Vec<Event>
    where
        K: AsRef<[u8]>,
        I: IntoIterator<Item = K>,
    {
        self.db
            .multi_get(self.event_cf(), ids)
            .flatten()
            .flatten()
            .filter_map(|slice| self.db.deserialize(&slice).ok())
            .collect()
    }

    pub fn flush(&self) {
        self.db.flush();
    }
}
