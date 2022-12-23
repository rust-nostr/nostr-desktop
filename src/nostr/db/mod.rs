// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::path::Path;
use std::sync::Arc;

use nostr_sdk::nostr::Contact;
use nostr_sdk::nostr::{secp256k1::XOnlyPublicKey, Event};
use nostr_sdk::Result;

pub mod model;
mod rocksdb;
mod util;

use self::model::Profile;
use self::rocksdb::{BoundColumnFamily, Store as RocksStore, WriteBatch, WriteSerializedBatch};

#[derive(Debug, Clone)]
pub struct Store {
    db: RocksStore,
}

const EVENT_CF: &str = "event";
const AUTHOR_CF: &str = "author";
const CONTACT_CF: &str = "contact";
const PROFILE_CF: &str = "profile";
const CHAT_CF: &str = "chat";
const CHANNEL_CF: &str = "channel";

const COLUMN_FAMILIES: &[&str] = &[
    EVENT_CF, AUTHOR_CF, CONTACT_CF, PROFILE_CF, CHAT_CF, CHANNEL_CF,
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

    pub fn save_events(&self, events: Vec<Event>) -> Result<()> {
        let mut batch = WriteBatch::default();

        for event in events.iter() {
            batch.put_serialized(self.event_cf(), util::event_prefix(event.id)?, event)?;
        }

        Ok(self.db.write(batch)?)
    }

    pub fn get_events(&self) -> Result<Vec<Event>> {
        Ok(self.db.iterator_value_serialized(self.event_cf())?)
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

    pub fn get_contacts(&self) -> Result<Vec<Contact>> {
        Ok(self.db.iterator_value_serialized(self.contact_cf())?)
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

    pub fn flush(&self) {
        self.db.flush();
    }
}
