// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Column, Text};
use iced::{Command, Element};
use nostr_sdk::nostr::Event;
use rocksdb::Direction;

use crate::message::{DashboardMessage, Message};
use crate::nostr::db::model::TextNote;
use crate::stage::dashboard::component::Dashboard;
use crate::stage::dashboard::{Context, State};

const FEED_LIMIT: usize = 40;

#[derive(Debug, Clone)]
pub enum HomeMessage {
    PushTextNote(Event),
}

#[derive(Debug, Default)]
pub struct HomeState {
    loaded: bool,
    last_timestamp: u64,
    notes: Vec<TextNote>,
    latest_offset: f32,
}

impl HomeState {
    pub fn new() -> Self {
        Self {
            loaded: false,
            last_timestamp: 0,
            notes: Vec::with_capacity(FEED_LIMIT),
            latest_offset: 0.0,
        }
    }
}

impl State for HomeState {
    fn title(&self) -> String {
        String::from("Nostr - Home")
    }

    fn load(&mut self, ctx: &Context) -> Command<Message> {
        self.loaded = true;
        self.notes = ctx.store.get_textnotes_with_limit(FEED_LIMIT);
        if let Some(note) = self.notes.last() {
            self.last_timestamp = note.timestamp;
        }
        Command::perform(async {}, |_| Message::Tick)
    }

    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message> {
        if !self.loaded {
            self.load(ctx);
        }

        match message {
            Message::Scrolled(offset) => {
                self.latest_offset = offset;

                if offset > 0.9 {
                    let more_notes = ctx.store.get_textnotes_from_timestamp(
                        self.last_timestamp,
                        Direction::Reverse,
                        FEED_LIMIT,
                    );
                    self.notes = [&self.notes, &more_notes[1..]].concat();
                }
            }
            Message::Dashboard(DashboardMessage::Home(msg)) => match msg {
                HomeMessage::PushTextNote(event) => {
                    if self.notes.len() > FEED_LIMIT {
                        self.notes.pop();
                    }
                    self.notes.push(TextNote::from(event));
                    self.notes.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                }
            },
            _ => (),
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
