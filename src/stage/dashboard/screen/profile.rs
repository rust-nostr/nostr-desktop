// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::Column;
use iced::{Command, Element};

use crate::message::{DashboardMessage, Message};
use crate::stage::dashboard::component::Dashboard;
use crate::stage::dashboard::context::Context;
use crate::stage::dashboard::State;

#[derive(Debug, Clone)]
pub enum ProfileMessage {}

#[derive(Debug, Default)]
pub struct ProfileState {}

impl ProfileState {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for ProfileState {
    fn title(&self) -> String {
        String::from("Nostr - Profile")
    }

    fn update(&mut self, _ctx: &mut Context, message: Message) -> Command<Message> {
        if let Message::Dashboard(DashboardMessage::Profile(_msg)) = message {
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

impl From<ProfileState> for Box<dyn State> {
    fn from(s: ProfileState) -> Box<dyn State> {
        Box::new(s)
    }
}
