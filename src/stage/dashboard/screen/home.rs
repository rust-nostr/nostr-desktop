// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Button, Column, Container, Row, Rule, Text};
use iced::{Command, Element};
use nostr_sdk::nostr::Event;

use crate::component::Icon;
use crate::message::{DashboardMessage, Message};
use crate::stage::dashboard::component::Dashboard;
use crate::stage::dashboard::{Context, State};
use crate::theme::icon::{CHAT, HEART, REPEAT};

const FEED_LIMIT: usize = 40;

#[derive(Debug, Clone)]
pub enum HomeMessage {
    PushTextNote(Event),
    Like(Event),
}

#[derive(Clone, Default)]
pub struct HomeState {
    loaded: bool,
    feed_ids: Vec<Vec<u8>>,
    latest_offset: f32,
    page: usize,
}

impl HomeState {
    pub fn new() -> Self {
        Self {
            loaded: false,
            feed_ids: Vec::new(),
            latest_offset: 0.0,
            page: 0,
        }
    }
}

impl State for HomeState {
    fn title(&self) -> String {
        String::from("Nostr - Home")
    }

    fn load(&mut self, ctx: &Context) -> Command<Message> {
        self.loaded = true;
        self.feed_ids = ctx.store.get_feed_ids(FEED_LIMIT, self.page as usize);
        Command::perform(async {}, |_| Message::Tick)
    }

    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message> {
        if !self.loaded {
            self.load(ctx);
        }

        match message {
            Message::Scrolled(offset) => {
                self.latest_offset = offset;

                if offset < 0.1 && self.page > 0 {
                    self.page -= 1;
                    self.feed_ids.truncate((self.page + 1) * FEED_LIMIT);
                    //return scrollable::snap_to(scrollable::Id::new("id"), 0.65);
                } else if offset > 0.9 && self.page * FEED_LIMIT < self.feed_ids.len() {
                    self.page += 1;
                    let mut more_notes = ctx.store.get_feed_ids(FEED_LIMIT, self.page);
                    if !self.feed_ids.ends_with(&more_notes) {
                        self.feed_ids.append(&mut more_notes);
                    }
                    //return scrollable::snap_to(scrollable::Id::new("id"), 0.35);
                }
            }
            Message::Dashboard(DashboardMessage::Home(msg)) => match msg {
                HomeMessage::PushTextNote(event) => {
                    self.feed_ids.push(event.created_at.to_be_bytes().to_vec());
                    self.feed_ids.sort_by(|a, b| b.cmp(a));
                }
                HomeMessage::Like(event) => {
                    let client = ctx.client.clone();
                    return Command::perform(async move { client.like(&event).await }, |_| {
                        Message::Tick
                    });
                }
            },
            _ => (),
        }

        Command::none()
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let mut content: Column<Message> = Column::new();

        let ids = self
            .feed_ids
            .iter()
            /* .skip(self.page.saturating_sub(1) * FEED_LIMIT)
            .take(FEED_LIMIT * 2) */;

        for note in ctx.store.get_feed_by_ids(ids).into_iter() {
            let display_name = if let Ok(profile) = ctx.store.get_profile(note.pubkey) {
                profile.metadata.display_name.unwrap_or_else(|| {
                    let pk = note.pubkey.to_string();
                    format!("{}:{}", &pk[0..8], &pk[pk.len() - 8..])
                })
            } else {
                let pk = note.pubkey.to_string();
                format!("{}:{}", &pk[0..8], &pk[pk.len() - 8..])
            };

            let buttons = Row::new()
                .push(Button::new(Icon::view(&CHAT)))
                .push(Button::new(Icon::view(&REPEAT)))
                .push(Button::new(Icon::view(&HEART)))
                .spacing(20);

            let post = Column::new()
                .push(Row::new().push(Text::new(display_name)))
                .push(Row::new().push(Text::new(note.content.clone())))
                .push(buttons)
                .push(Rule::horizontal(1))
                .spacing(10);

            let post = Container::new(post).padding(15);

            content = content.push(post);
        }

        Dashboard::new().view(ctx, content.spacing(10).padding(20))
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
