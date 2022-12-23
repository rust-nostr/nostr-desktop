// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::{executor, Application, Command, Element, Settings, Subscription, Theme};

mod component;
mod error;
mod message;
mod nostr;
mod stage;
mod theme;
mod util;

use self::message::Message;

pub fn main() -> iced::Result {
    env_logger::init();
    let mut settings = Settings::default();
    settings.window.min_size = Some((600, 600));
    NostrDesktop::run(settings)
}

pub enum NostrDesktop {
    Auth(stage::Auth),
    Dashboard(stage::App),
}

impl Application for NostrDesktop {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        // read local db
        // if key exists, load main app
        // else load login/register view
        let stage = stage::Auth::new();
        (Self::Auth(stage.0), stage.1)
    }

    fn title(&self) -> String {
        match self {
            Self::Auth(auth) => auth.title(),
            Self::Dashboard(app) => app.title(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match self {
            Self::Auth(auth) => auth.subscription(),
            Self::Dashboard(app) => app.subscription(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match self {
            Self::Auth(auth) => {
                let (command, stage_to_move) = auth.update(message);
                if let Some(stage) = stage_to_move {
                    *self = stage;
                }
                command
            }
            Self::Dashboard(app) => app.update(message),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match self {
            Self::Auth(auth) => auth.view(),
            Self::Dashboard(app) => app.view(),
        }
    }
}
