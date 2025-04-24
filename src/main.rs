// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::{executor, Application, Command, Element, Settings, Size, Subscription, Theme};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

mod component;
mod error;
mod message;
mod stage;
mod sync;
mod theme;
mod util;

use theme::font::BOOTSTRAP_ICONS_BYTES;

use self::message::Message;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Can't start Tokio runtime"));

pub fn main() -> iced::Result {
    env_logger::init();
    let mut settings = Settings {
        fonts: vec![BOOTSTRAP_ICONS_BYTES.into()],
        ..Settings::default()
    };
    settings.window.min_size = Some(Size {
        width: 600.0,
        height: 600.0,
    });
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
            Self::Dashboard(app) => match message {
                Message::Lock => {
                    let client = app.context.client.clone();
                    *self = Self::Auth(stage::Auth::new().0);
                    Command::perform(
                        async move {
                            if let Err(e) = client.shutdown().await {
                                log::error!("Impossible to shutdown client: {}", e);
                            }
                        },
                        |_| Message::Tick,
                    )
                }
                _ => app.update(message),
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match self {
            Self::Auth(auth) => auth.view(),
            Self::Dashboard(app) => app.view(),
        }
    }
}
