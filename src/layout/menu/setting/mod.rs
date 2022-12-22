// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Button, Column};
use iced::{Command, Element};

use crate::component::Dashboard;
use crate::context::{Context, Menu, Setting, Stage};
use crate::layout::State;
use crate::message::{MenuMessage, Message};

pub mod relays;

pub use self::relays::{RelaysMessage, RelaysState};

#[derive(Debug, Clone)]
pub enum SettingMessage {
    GoToRelays,
    Relays(RelaysMessage),
}

#[derive(Debug, Default)]
pub struct SettingState {}

impl SettingState {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for SettingState {
    fn title(&self) -> String {
        String::from("Nostr - Setting")
    }

    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message> {
        if let Some(_client) = ctx.client.as_mut() {
            if let Message::Menu(MenuMessage::Setting(msg)) = message {
                match msg {
                    SettingMessage::GoToRelays => Command::perform(async move {}, |_| {
                        Message::SetStage(Stage::Menu(Menu::Setting(Setting::Relays)))
                    }),
                    _ => Command::none(),
                }
            } else {
                Command::none()
            }
        } else {
            Command::perform(async move {}, |_| Message::SetStage(Stage::Login))
        }
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let button =
            Button::new("Relays")
                .padding(10)
                .on_press(Message::Menu(MenuMessage::Setting(
                    SettingMessage::GoToRelays,
                )));
        let content = Column::new().push(button);
        Dashboard::new().view(ctx, content)
    }
}

impl From<SettingState> for Box<dyn State> {
    fn from(s: SettingState) -> Box<dyn State> {
        Box::new(s)
    }
}
