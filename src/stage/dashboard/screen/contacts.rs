// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::Column;
use iced::{Command, Element};

use crate::message::{DashboardMessage, Message};
use crate::stage::dashboard::component::Dashboard;
use crate::stage::dashboard::{Context, State};

#[derive(Debug, Clone)]
pub enum ContactsMessage {}

#[derive(Debug, Default)]
pub struct ContactsState {}

impl ContactsState {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for ContactsState {
    fn title(&self) -> String {
        String::from("Nostr - Contacts")
    }

    fn update(&mut self, _ctx: &mut Context, message: Message) -> Command<Message> {
        if let Message::Dashboard(DashboardMessage::Contacts(_msg)) = message {
            Command::none()
        } else {
            Command::none()
        }
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let content = Column::new();
        Dashboard::new().view(ctx, content)
    }
}

impl From<ContactsState> for Box<dyn State> {
    fn from(s: ContactsState) -> Box<dyn State> {
        Box::new(s)
    }
}
