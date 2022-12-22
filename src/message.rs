// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

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
    SetStage(Stage),
    Clipboard(String),
    Login(LoginMessage),
    Menu(MenuMessage),
}
