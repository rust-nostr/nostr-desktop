// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use nostr_sdk::nostr::secp256k1::XOnlyPublicKey;
use nostr_sdk::nostr::{Event, Metadata, Tag};
use serde::{Deserialize, Serialize};

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

impl From<Event> for TextNote {
    fn from(event: Event) -> Self {
        Self {
            pubkey: event.pubkey,
            content: event.content,
            tags: event.tags,
            timestamp: event.created_at,
        }
    }
}
