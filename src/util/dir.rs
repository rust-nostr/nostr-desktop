// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::fs;
use std::path::{Path, PathBuf};

use nostr_sdk::Result;

pub fn home() -> PathBuf {
    match dirs::home_dir() {
        Some(path) => path,
        None => Path::new("./").to_path_buf(),
    }
}

pub fn default_dir() -> Result<PathBuf> {
    let path: PathBuf = home().join(".nostr-desktop");
    fs::create_dir_all(path.as_path())?;
    Ok(path)
}
