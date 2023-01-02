// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::path::Path;
use std::str::FromStr;

use nostr_sdk::nostr::secp256k1::schnorr::Signature;
use nostr_sdk::nostr::secp256k1::XOnlyPublicKey;
use nostr_sdk::nostr::{Contact, Event, Kind, Sha256Hash};
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

    pub fn save_event(&self, event: &Event) -> Result<(), Error> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT OR IGNORE INTO profile (pubkey) VALUES (?)",
            [event.pubkey.to_string()],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO event (id, pubkey, created_at, kind, tags, content, sig) VALUES (?, ?, ?, ?, ?, ?, ?)",
            (
                event.id.to_string(),
                event.pubkey.to_string(),
                event.created_at,
                event.kind.as_u64(),
                self.serialize(event.tags.clone())?,
                event.content.clone(),
                event.sig.to_string(),
            ),
        )?;

        Ok(())
    }

    pub fn insert_profile(&self, profile: Profile) -> Result<(), Error> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO profile (pubkey, name, display_name, about, website, picture, nip05, lud06, lud16, is_contact, metadata_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);",
            (
                profile.pubkey.to_string(),
                profile.name,
                profile.display_name,
                profile.about,
                profile.website,
                profile.picture,
                profile.nip05,
                profile.lud06,
                profile.lud16,
                profile.is_contact,
                profile.metadata_at,
            ),
        )?;

        Ok(())
    }

    pub fn update_profile(&self, profile: Profile) -> Result<(), Error> {
        let conn = self.pool.get()?;
        conn.execute(
            "UPDATE profile SET name=?, display_name=?, about=?, website=?, picture=?, nip05=?, lud06=?, lud16=?, metadata_at=? WHERE pubkey = ?;",
            (
                profile.name,
                profile.display_name,
                profile.about,
                profile.website,
                profile.picture,
                profile.nip05,
                profile.lud06,
                profile.lud16,
                profile.metadata_at,
                profile.pubkey.to_string(),
            ),
        )?;

        Ok(())
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
                    is_contact: row.get(9)?,
                    metadata_at: row.get(10)?,
                })
            }
            None => Err(Error::ValueNotFound),
        }
    }

    pub fn set_contacts(&self, list: Vec<Contact>) -> Result<(), Error> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("INSERT INTO profile (pubkey, is_contact) VALUES(?, ?) ON CONFLICT(pubkey) DO UPDATE SET is_contact=excluded.is_contact;")?;
        for contact in list.into_iter() {
            stmt.execute((contact.pk.to_string(), true))?;
        }
        Ok(())
    }

    pub fn get_contacts(&self) -> Result<Vec<Profile>, Error> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT * FROM profile WHERE is_contact = ?")?;
        let mut rows = stmt.query([true])?;

        let mut profiles = Vec::new();

        while let Ok(Some(row)) = rows.next() {
            let pubkey: String = row.get(0)?;
            profiles.push(Profile {
                pubkey: XOnlyPublicKey::from_str(&pubkey)?,
                name: row.get(1)?,
                display_name: row.get(2)?,
                about: row.get(3)?,
                website: row.get(4)?,
                picture: row.get(5)?,
                nip05: row.get(6)?,
                lud06: row.get(7)?,
                lud16: row.get(8)?,
                is_contact: row.get(9)?,
                metadata_at: row.get(10)?,
            })
        }

        Ok(profiles)
    }

    pub fn set_authors(&self, authors: Vec<XOnlyPublicKey>) -> Result<(), Error> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("INSERT OR IGNORE INTO profile (pubkey) VALUES (?)")?;

        for author in authors.into_iter() {
            stmt.insert([author.to_string()])?;
        }

        Ok(())
    }

    pub fn get_authors(&self) -> Result<Vec<XOnlyPublicKey>, Error> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT pubkey FROM profile")?;
        let mut rows = stmt.query([])?;

        let mut authors = Vec::new();

        while let Ok(Some(row)) = rows.next() {
            let pubkey: String = row.get(0)?;
            authors.push(XOnlyPublicKey::from_str(&pubkey)?);
        }

        Ok(authors)
    }

    pub fn insert_relay(&self, url: String, proxy: Option<String>) -> Result<(), Error> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO relays (url, proxy) VALUES (?, ?);",
            (url, proxy),
        )?;

        Ok(())
    }

    pub fn get_feed(&self, limit: usize, _page: usize) -> Result<Vec<Event>> {
        let conn = self.pool.get()?;
        let mut stmt =
            conn.prepare("SELECT * FROM event WHERE kind = ? ORDER BY created_at DESC LIMIT ?")?;
        let mut rows = stmt.query([1, limit])?;

        let mut events = Vec::new();

        while let Ok(Some(row)) = rows.next() {
            let id: String = row.get(0)?;
            let pubkey: String = row.get(1)?;
            let tags: Vec<u8> = row.get(4)?;
            let sig: String = row.get(6)?;
            events.push(Event {
                id: Sha256Hash::from_str(&id)?,
                pubkey: XOnlyPublicKey::from_str(&pubkey)?,
                created_at: row.get(2)?,
                kind: Kind::Custom(row.get(3)?),
                tags: self.deserialize(&tags)?,
                content: row.get(5)?,
                ots: None,
                sig: Signature::from_str(&sig)?,
            })
        }

        Ok(events)
    }
}
