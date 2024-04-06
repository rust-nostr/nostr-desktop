// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::scrollable;
use nostr_sdk::nostr::Event;
use nostr_sdk::Client;

use crate::stage::auth::screen::LoginMessage;
use crate::stage::dashboard::screen::{
    ChatMessage, ContactsMessage, ExploreMessage, HomeMessage, NotificationsMessage,
    ProfileMessage, SettingMessage,
};
use crate::stage::{auth, dashboard};

#[derive(Debug, Clone)]
pub enum DashboardMessage {
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
    Scrolled(scrollable::Viewport),
    SetAuthStage(auth::Stage),
    SetDashboardStage(dashboard::Stage),
    LoginResult(Client),
    Lock,
    Clipboard(String),
    Login(LoginMessage),
    Dashboard(DashboardMessage),
}
