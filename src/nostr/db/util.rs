// Copyright (c) 2021-2022 Yuki Kishimoto
// Distributed under the MIT software license

use nostr_sdk::nostr::Sha256Hash;

const HASH_PREFIX_LEN: usize = 8;

pub type HashPrefix = [u8; HASH_PREFIX_LEN];

pub fn hash_prefix(event_id: Sha256Hash) -> HashPrefix {
    let mut prefix = HashPrefix::default();
    prefix.copy_from_slice(&event_id[..HASH_PREFIX_LEN]);
    prefix
}
