// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use nostr_sdk::client::blocking::Client;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Setting {
    Main,
    Relays,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    Home,
    Explore,
    Chats,
    Contacts,
    Notifications,
    Profile,
    Setting(Setting),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Login,
    Register,
    Menu(Menu),
}

impl Default for Stage {
    fn default() -> Self {
        Self::Login
    }
}

pub struct Context {
    //pub config: ConfigContext,
    pub stage: Stage,
    pub client: Option<Client>,
}

impl Context {
    pub fn new(stage: Stage, client: Option<Client>) -> Self {
        Self { stage, client }
    }

    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
    }

    pub fn set_client(&mut self, client: Option<Client>) {
        self.client = client;
    }
}
