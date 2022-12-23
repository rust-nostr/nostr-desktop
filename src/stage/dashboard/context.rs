// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use nostr_sdk::blocking::Client;

use crate::nostr::db::Store;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Setting {
    Main,
    Relays,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Home,
    Explore,
    Chats,
    Contacts,
    Notifications,
    Profile,
    Setting(Setting),
}

impl Default for Stage {
    fn default() -> Self {
        Self::Home
    }
}

pub struct Context {
    //pub config: ConfigContext,
    pub stage: Stage,
    pub client: Client,
    pub store: Store,
}

impl Context {
    pub fn new(stage: Stage, client: Client, store: Store) -> Self {
        Self {
            stage,
            client,
            store,
        }
    }

    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
    }

    pub fn set_client(&mut self, client: Client) {
        self.client = client;
    }

    pub fn set_store(&mut self, store: Store) {
        self.store = store;
    }
}
