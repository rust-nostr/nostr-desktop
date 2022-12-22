// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::Column;
use iced::{Command, Element};

use crate::component::Dashboard;
use crate::context::{Context, Stage};
use crate::layout::State;
use crate::message::{MenuMessage, Message};

#[derive(Debug, Clone)]
pub enum ChatMessage {}

#[derive(Debug, Default)]
pub struct ChatState {}

impl ChatState {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for ChatState {
    fn title(&self) -> String {
        String::from("Nostr - Chat")
    }

    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message> {
        if let Some(_client) = ctx.client.as_mut() {
            if let Message::Menu(MenuMessage::Chat(_msg)) = message {
                Command::none()
            } else {
                Command::none()
            }
        } else {
            Command::perform(async move {}, |_| Message::SetStage(Stage::Login))
        }
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let content = Column::new();
        Dashboard::new().view(ctx, content)
    }
}

impl From<ChatState> for Box<dyn State> {
    fn from(s: ChatState) -> Box<dyn State> {
        Box::new(s)
    }
}
