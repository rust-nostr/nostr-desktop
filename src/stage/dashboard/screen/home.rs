// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::collections::VecDeque;

use iced::widget::{Column, Text};
use iced::{Command, Element};
use nostr_sdk::nostr::Event;

use crate::message::{DashboardMessage, Message};
use crate::nostr::db::model::TextNote;
use crate::stage::dashboard::component::Dashboard;
use crate::stage::dashboard::{Context, State};

const FEED_LIMIT: usize = 30;

#[derive(Debug, Clone)]
pub enum HomeMessage {
    PushTextNote(Event),
}

#[derive(Debug, Default)]
pub struct HomeState {
    loaded: bool,
    notes: VecDeque<TextNote>,
}

impl HomeState {
    pub fn new() -> Self {
        Self {
            loaded: false,
            notes: VecDeque::with_capacity(FEED_LIMIT),
        }
    }
}

impl State for HomeState {
    fn title(&self) -> String {
        String::from("Nostr - Home")
    }

    fn load(&mut self, ctx: &Context) -> Command<Message> {
        self.loaded = true;
        if let Ok(notes) = ctx.store.get_textnotes_with_limit(FEED_LIMIT) {
            self.notes = notes.into();
            Command::perform(async {}, |_| Message::Tick)
        } else {
            Command::none()
        }
    }

    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message> {
        if !self.loaded {
            self.load(ctx);
        }

        if let Message::Dashboard(DashboardMessage::Home(msg)) = message {
            match msg {
                HomeMessage::PushTextNote(event) => {
                    if self.notes.len() > FEED_LIMIT {
                        self.notes.pop_back();
                    }
                    self.notes.push_front(TextNote::from(event));
                }
            };
        }

        Command::none()
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let mut content = Column::new();

        for note in self.notes.iter() {
            content = content.push(Text::new(note.content.clone()));
        }

        Dashboard::new().view(ctx, content)
    }
}

impl From<HomeState> for Box<dyn State> {
    fn from(s: HomeState) -> Box<dyn State> {
        Box::new(s)
    }
}

impl From<HomeMessage> for Message {
    fn from(msg: HomeMessage) -> Self {
        Self::Dashboard(DashboardMessage::Home(msg))
    }
}
