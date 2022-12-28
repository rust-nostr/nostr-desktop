// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::path::Path;
use std::str::FromStr;

use nostr_sdk::nostr::secp256k1::XOnlyPublicKey;
use nostr_sdk::nostr::{Contact, Event, Kind, KindBase, Sha256Hash};
use nostr_sdk::Result;
use r2d2;
use r2d2_sqlite::SqliteConnectionManager;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod model;
mod schema;

use self::model::Profile;

pub type SqlitePool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type PooledConnection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub const DB_NAME: &str = "nostr.db";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Pool error: {0}")]
    Pool(#[from] r2d2::Error),
    #[error("Secp256k1 error: {0}")]
    Secp256k1(#[from] nostr_sdk::nostr::secp256k1::Error),
    #[error("Impossible to deserialize")]
    FailedToDeserialize,
    #[error("Impossible to serialize")]
    FailedToSerialize,
    #[error("Value not found")]
    ValueNotFound,
}

#[derive(Debug, Clone)]
pub struct Store {
    pool: SqlitePool,
}

impl Store {
    pub fn open<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let manager = SqliteConnectionManager::file(path);
        let pool = r2d2::Pool::new(manager)?;

        schema::upgrade_db(&mut pool.get()?)?;

        Ok(Self { pool })
    }

    fn serialize<T>(&self, data: T) -> Result<Vec<u8>, Error>
    where
        T: Serialize + std::fmt::Debug,
    {
        match serde_json::to_string(&data) {
            Ok(serialized) => Ok(serialized.into_bytes()),
            Err(_) => Err(Error::FailedToSerialize),
        }
    }

    fn deserialize<T>(&self, data: &[u8]) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        match serde_json::from_slice::<T>(data) {
            Ok(u) => Ok(u),
            Err(_) => Err(Error::FailedToDeserialize),
        }
    }

    pub fn get_event(&self, event_id: Sha256Hash) -> Result<Event, Error> {
        todo!()
    }

    pub fn save_event(&self, event: &Event) -> Result<(), Error> {
        todo!()
    }

    pub fn set_profile(&self, profile: Profile) -> Result<(), Error> {
        todo!()
    }

    pub fn get_profile(&self, public_key: XOnlyPublicKey) -> Result<Profile, Error> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT * FROM profile WHERE pubkey = ?")?;
        let mut rows = stmt.query([public_key.to_string()])?;

        match rows.next()? {
            Some(row) => {
                let pubkey: String = row.get(0)?;
                Ok(Profile {
                    pubkey: XOnlyPublicKey::from_str(&pubkey)?,
                    name: row.get(1)?,
                    display_name: row.get(2)?,
                    about: row.get(3)?,
                    website: row.get(4)?,
                    picture: row.get(5)?,
                    nip05: row.get(6)?,
                    lud06: row.get(7)?,
                    lud16: row.get(8)?,
                    metadata_at: row.get(9)?,
                })
            }
            None => Err(Error::ValueNotFound),
        }
    }

    pub fn set_contacts(&self, list: Vec<Contact>) -> Result<(), Error> {
        todo!()
    }

    pub fn get_contacts(&self) -> Vec<Contact> {
        todo!()
    }

    pub fn set_author(&self, public_key: XOnlyPublicKey) -> Result<(), Error> {
        todo!()
    }

    pub fn set_authors(&self, authors: Vec<XOnlyPublicKey>) -> Result<(), Error> {
        todo!()
    }

    pub fn get_authors(&self) -> Result<Vec<XOnlyPublicKey>, Error> {
        todo!()
    }

    pub fn get_feed(&self, limit: usize, page: usize) -> Vec<Event> {
        todo!()
    }
}

/* pub fn copy_events(store: &Store) {
    let conn = Connection::open("./nostr.db").unwrap();

    println!("Copying events...");

    conn.execute(
        "CREATE TABLE event (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            event_id BLOB NOT NULL,
            pubkey TEXT NOT NULL,
            created_at UNSIGNED BIG INT,
            kind UNSIGNED BIG INT,
            tags BLOB,
            content TEXT NOT NULL,
            sig TEXT
        )",
        (), // empty list of parameters.
    ).unwrap();

    for event in store.get_events().into_iter() {
        let tags = store.db.serialize(event.tags).unwrap();
        conn.execute(
            "INSERT INTO event (event_id, pubkey, created_at, kind, tags, content, sig) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (&event.id.to_vec(), &event.pubkey.to_string(), &event.created_at, 1, tags, &event.content, &event.sig.to_string()),
        ).unwrap();
    }

    println!("Events copied");
}

pub fn get_feed() -> Vec<Vec<u8>> {
    let conn = Connection::open("./nostr.db").unwrap();


    let mut stmt = conn.prepare("SELECT event_id FROM event WHERE kind = 1 ORDER BY created_at DESC LIMIT 1000").unwrap();
    let person_iter = stmt.query_map([], |row| {
        let event_id: Vec<u8> = row.get(0).unwrap();
        let mut prefix = [0u8; 8];
        prefix.copy_from_slice(&event_id[..8]);
        Ok(prefix.to_vec())
    }).unwrap();

    person_iter.map(|r| r.unwrap()).collect()
} */
