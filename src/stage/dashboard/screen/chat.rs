// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Column, Row, Text};
use iced::{Command, Element};
use nostr_sdk::nostr::Event;

use crate::message::Message;
use crate::stage::dashboard::component::Dashboard;
use crate::stage::dashboard::{Context, State};

#[derive(Debug, Clone)]
pub enum ChatMessage {}

#[derive(Debug, Default)]
pub struct ChatState {
    events: Vec<Event>,
    error: Option<String>,
}

impl ChatState {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            error: None,
        }
    }

    pub fn clear(&mut self) {
        self.events = Vec::new();
        self.error = None;
    }
}

impl State for ChatState {
    fn title(&self) -> String {
        String::from("Nostr - Chat")
    }

    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message> {
        self.events = ctx.store.get_events().unwrap();

        if let Message::Sync(event) = message {
            self.events.push(event);
        }

        Command::none()
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let mut messages = Column::new().spacing(10);

        for event in self.events.iter() {
            messages = messages.push(Row::new().push(Text::new(&event.content)));
        }

        let content = Column::new()
            .push(Row::new().push(if let Some(error) = &self.error {
                Row::new().push(Text::new(error))
            } else {
                Row::new()
            }))
            .push(messages);

        Dashboard::new().view(ctx, content)
    }
}

impl From<ChatState> for Box<dyn State> {
    fn from(s: ChatState) -> Box<dyn State> {
        Box::new(s)
    }
}
