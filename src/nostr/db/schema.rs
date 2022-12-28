// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::cmp::Ordering;

use const_format::formatcp;
use rusqlite::Connection;

use super::{Error, PooledConnection};

/// Startup DB Pragmas
pub const STARTUP_SQL: &str = r##"
PRAGMA main.synchronous=NORMAL;
PRAGMA foreign_keys = ON;
PRAGMA journal_size_limit=32768;
pragma mmap_size = 17179869184; -- cap mmap at 16GB
"##;

/// Latest database version
pub const DB_VERSION: usize = 1;

/// Schema definition
const INIT_SQL: &str = formatcp!(
    r##"
-- Database settings
PRAGMA encoding = "UTF-8";
PRAGMA journal_mode=WAL;
PRAGMA main.synchronous=NORMAL;
PRAGMA foreign_keys = ON;
PRAGMA application_id = 1654008667;
PRAGMA user_version = {};

-- Event Table
CREATE TABLE IF NOT EXISTS event (
id BLOB PRIMARY KEY,
pubkey TEXT NOT NULL,
created_at INTEGER NOT NULL,
kind INTEGER NOT NULL,
content TEXT NOT NULL,
FOREIGN KEY(pubkey) REFERENCES profile(pubkey)
);

-- Event Indexes
CREATE INDEX IF NOT EXISTS pubkey_index ON event(pubkey);
CREATE INDEX IF NOT EXISTS created_at_index ON event(created_at);
CREATE INDEX IF NOT EXISTS event_composite_index ON event(kind,created_at);

-- Tag Table
-- Tag values are stored as either a BLOB (if they come in as a
-- hex-string), or TEXT otherwise.
-- This means that searches need to select the appropriate column.
CREATE TABLE IF NOT EXISTS tag (
id INTEGER PRIMARY KEY,
event_id BLOB NOT NULL, -- an event ID that contains a tag.
kind TEXT, -- the tag kind ("p", "e", whatever)
value BLOB, -- the tag value,
FOREIGN KEY(event_id) REFERENCES event(id) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS tag_val_index ON tag(value);
CREATE INDEX IF NOT EXISTS tag_composite_index ON tag(event_id,name,value);

-- Author Table (author seen)
CREATE TABLE IF NOT EXISTS author (
pubkey TEXT PRIMARY KEY NOT NULL,
);

-- Profile Table
CREATE TABLE IF NOT EXISTS profile (
pubkey TEXT PRIMARY KEY NOT NULL,
name TEXT DEFAULT NULL,
display_name TEXT DEFAULT NULL,
about TEXT DEFAULT NULL,
website TEXT DEFAULT NULL,
picture TEXT DEFAULT NULL,
nip05 TEXT DEFAULT NULL,
lud06 TEXT DEFAULT NULL,
lud16 TEXT DEFAULT NULL,
metadata_at INTEGER DEFAULT NULL
);

-- Contact Table
CREATE TABLE IF NOT EXISTS contact (
pubkey TEXT PRIMARY KEY NOT NULL,
relay_url TEXT DEFAULT NULL,
alias TEXT DEFAULT NULL
);
"##,
    DB_VERSION
);

/// Determine the current application database schema version.
pub fn curr_db_version(conn: &mut Connection) -> Result<usize, Error> {
    let query = "PRAGMA user_version;";
    let curr_version = conn.query_row(query, [], |row| row.get(0))?;
    Ok(curr_version)
}

fn mig_init(conn: &mut PooledConnection) -> Result<usize, Error> {
    match conn.execute_batch(INIT_SQL) {
        Ok(()) => {
            log::info!(
                "database pragma/schema initialized to v{}, and ready",
                DB_VERSION
            );
        }
        Err(err) => {
            log::error!("update failed: {}", err);
            panic!("database could not be initialized");
        }
    }
    Ok(DB_VERSION)
}

/// Upgrade DB to latest version, and execute pragma settings
pub fn upgrade_db(conn: &mut PooledConnection) -> Result<(), Error> {
    // check the version.
    let mut curr_version = curr_db_version(conn)?;
    log::info!("DB version = {:?}", curr_version);

    match curr_version.cmp(&DB_VERSION) {
        // Database is new or not current
        Ordering::Less => {
            // initialize from scratch
            if curr_version == 1 {
                curr_version = mig_init(conn)?;
            }

            // for initialized but out-of-date schemas, proceed to
            // upgrade sequentially until we are current.
            /* if curr_version == 1 {
                curr_version = mig_1_to_2(conn)?;
            } */

            if curr_version == DB_VERSION {
                log::info!(
                    "All migration scripts completed successfully.  Welcome to v{}.",
                    DB_VERSION
                );
            }
        }
        // Database is current, all is good
        Ordering::Equal => {
            log::debug!("Database version was already current (v{})", DB_VERSION);
        }
        // Database is newer than what this code understands, abort
        Ordering::Greater => {
            panic!(
                "Database version is newer than supported by this executable (v{} > v{})",
                curr_version, DB_VERSION
            );
        }
    }

    // Setup PRAGMA
    conn.execute_batch(STARTUP_SQL)?;
    log::debug!("SQLite PRAGMA startup completed");
    Ok(())
}
