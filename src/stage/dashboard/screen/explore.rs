// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::Column;
use iced::{Command, Element};

use crate::message::{DashboardMessage, Message};
use crate::stage::dashboard::component::Dashboard;
use crate::stage::dashboard::{Context, State};

#[derive(Debug, Clone)]
pub enum ExploreMessage {}

#[derive(Debug, Default)]
pub struct ExploreState {}

impl ExploreState {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for ExploreState {
    fn title(&self) -> String {
        String::from("Nostr - Explore")
    }

    fn update(&mut self, _ctx: &mut Context, message: Message) -> Command<Message> {
        if let Message::Dashboard(DashboardMessage::Explore(_msg)) = message {
            Command::none()
        } else {
            Command::none()
        }
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let content = Column::new();
        Dashboard::new().view(ctx, content.spacing(20).padding(20))
    }
}

impl From<ExploreState> for Box<dyn State> {
    fn from(s: ExploreState) -> Box<dyn State> {
        Box::new(s)
    }
}
