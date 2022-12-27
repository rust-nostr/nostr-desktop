// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use nostr_sdk::nostr::Metadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub metadata: Metadata,
    pub timestamp: u64,
}
