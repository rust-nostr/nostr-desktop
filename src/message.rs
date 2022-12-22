// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use nostr_sdk::nostr::Event;

use crate::context::Stage;
use crate::layout::{
    ChatMessage, ContactsMessage, ExploreMessage, HomeMessage, LoginMessage, NotificationsMessage,
    ProfileMessage, SettingMessage,
};

#[derive(Debug, Clone)]
pub enum MenuMessage {
    Home(HomeMessage),
    Explore(ExploreMessage),
    Chat(ChatMessage),
    Contacts(ContactsMessage),
    Notifications(NotificationsMessage),
    Profile(ProfileMessage),
    Setting(SettingMessage),
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Sync(Event),
    SetStage(Stage),
    Clipboard(String),
    Login(LoginMessage),
    Menu(MenuMessage),
}
