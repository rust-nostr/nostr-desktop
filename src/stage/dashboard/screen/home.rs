// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::Column;
use iced::{Command, Element};
use nostr_sdk::nostr::Event;

use crate::message::{DashboardMessage, Message};
use crate::stage::dashboard::component::post::Post;
use crate::stage::dashboard::component::Dashboard;
use crate::stage::dashboard::{Context, State};

const FEED_LIMIT: usize = 40;

#[derive(Debug, Clone)]
pub enum HomeMessage {
    PushTextNote(Event),
    Like(Event),
}

#[derive(Clone, Default)]
pub struct HomeState {
    loaded: bool,
    latest_offset: f32,
    page: usize,
}

impl HomeState {
    pub fn new() -> Self {
        Self {
            loaded: false,
            latest_offset: 0.0,
            page: 1,
        }
    }
}

impl State for HomeState {
    fn title(&self) -> String {
        String::from("Nostr - Home")
    }

    fn load(&mut self, _ctx: &Context) -> Command<Message> {
        self.loaded = true;
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
                    self.page += 1;
                }
            }
            Message::Dashboard(DashboardMessage::Home(msg)) => match msg {
                HomeMessage::PushTextNote(_) => {}
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

        if let Ok(store) = ctx.client.store() {
            for event in store
                .get_feed(FEED_LIMIT, self.page)
                .unwrap_or_default()
                .into_iter()
            {
                content = content.push(Post::new(event).view(ctx));
            }
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
