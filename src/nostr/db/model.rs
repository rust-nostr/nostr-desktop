// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use nostr_sdk::nostr::secp256k1::XOnlyPublicKey;
use nostr_sdk::nostr::{Event as NostrEvent, Metadata, Tag};
use serde::{Deserialize, Serialize};

/* #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Event {
    pub pubkey: XOnlyPublicKey,
    pub created_at: u64,
    pub kind: Kind,
    pub tags: Vec<Tag>,
    pub content: String,
}

impl From<NostrEvent> for Event {
    fn from(event: NostrEvent) -> Self {
        Self {
            pubkey: event.pubkey,
            created_at: event.created_at,
            kind: event.kind,
            tags: event.tags,
            content: event.content,
        }
    }
} */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub metadata: Metadata,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TextNote {
    pub pubkey: XOnlyPublicKey,
    pub content: String,
    pub tags: Vec<Tag>,
    pub timestamp: u64,
}

impl From<NostrEvent> for TextNote {
    fn from(event: NostrEvent) -> Self {
        Self {
            pubkey: event.pubkey,
            content: event.content,
            tags: event.tags,
            timestamp: event.created_at,
        }
    }
}
