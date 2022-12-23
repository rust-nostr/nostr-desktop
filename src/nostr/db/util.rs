// Copyright (c) 2021-2022 Yuki Kishimoto
// Distributed under the MIT software license

use nostr_sdk::nostr::Sha256Hash;
use nostr_sdk::Result;

const HASH_PREFIX_LEN: usize = 8;

type HashPrefix = [u8; HASH_PREFIX_LEN];

pub fn event_prefix(event_id: Sha256Hash) -> Result<HashPrefix> {
    let prefix = <[u8; HASH_PREFIX_LEN]>::try_from(&event_id[..HASH_PREFIX_LEN])?;
    let value = u64::from_be_bytes(prefix);
    Ok(value.to_be_bytes())
}
