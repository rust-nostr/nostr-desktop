// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Column, Text};
use iced::{Command, Element};

use crate::message::Message;
use crate::nostr::db::model::TextNote;
use crate::stage::dashboard::component::Dashboard;
use crate::stage::dashboard::{Context, State};

#[derive(Debug, Clone)]
pub enum HomeMessage {}

#[derive(Debug, Default)]
pub struct HomeState {
    notes: Vec<TextNote>,
}

impl HomeState {
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }
}

impl State for HomeState {
    fn title(&self) -> String {
        String::from("Nostr - Home")
    }

    fn update(&mut self, ctx: &mut Context, _message: Message) -> Command<Message> {
        if self.notes.is_empty() {
            if let Ok(notes) = ctx.store.get_textnotes_with_limit(50) {
                self.notes = notes;
            }
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
