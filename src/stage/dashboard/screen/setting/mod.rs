// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Button, Column};
use iced::{Command, Element};

use crate::message::{DashboardMessage, Message};
use crate::stage::dashboard::component::Dashboard;
use crate::stage::dashboard::{Context, Setting, Stage, State};

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

    fn update(&mut self, _ctx: &mut Context, message: Message) -> Command<Message> {
        if let Message::Dashboard(DashboardMessage::Setting(msg)) = message {
            match msg {
                SettingMessage::GoToRelays => Command::perform(async move {}, |_| {
                    Message::SetDashboardStage(Stage::Setting(Setting::Relays))
                }),
                _ => Command::none(),
            }
        } else {
            Command::none()
        }
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let button = Button::new("Relays")
            .padding(10)
            .on_press(Message::Dashboard(DashboardMessage::Setting(
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
