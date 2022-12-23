// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::path::Path;
use std::sync::Arc;

use nostr_sdk::nostr::Event;
use nostr_sdk::Result;

pub mod model;
mod rocksdb;
mod util;

use self::rocksdb::{BoundColumnFamily, Store as RocksStore, WriteBatch, WriteSerializedBatch};

#[derive(Debug, Clone)]
pub struct Store {
    db: RocksStore,
}

const EVENT_CF: &str = "event";
const CONTACT_CF: &str = "contact";
const PROFILE_CF: &str = "profile";
const CHAT_CF: &str = "chat";
const CHANNEL_CF: &str = "channel";

const COLUMN_FAMILIES: &[&str] = &[EVENT_CF, CONTACT_CF, PROFILE_CF, CHAT_CF, CHANNEL_CF];

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

    /* fn contact_cf(&self) -> Arc<BoundColumnFamily> {
        self.db.cf_handle(CONTACT_CF)
    } */

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

    pub fn flush(&self) {
        self.db.flush();
    }
}
