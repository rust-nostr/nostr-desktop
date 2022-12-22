// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Button, Column};
use iced::{Command, Element};

use crate::component::Dashboard;
use crate::context::{Context, Menu, Setting, Stage};
use crate::layout::State;
use crate::Message;

#[derive(Debug, Clone)]
pub enum HomeMessage {
    GoToRelays,
}

#[derive(Debug, Default)]
pub struct HomeState {}

impl HomeState {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for HomeState {
    fn title(&self) -> String {
        String::from("Nostr - Home")
    }

    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message> {
        if let Some(_client) = ctx.client.as_mut() {
            if let Message::Home(msg) = message {
                match msg {
                    HomeMessage::GoToRelays => Command::perform(async move {}, |_| {
                        Message::SetStage(Stage::Menu(Menu::Setting(Setting::Relays)))
                    }),
                }
            } else {
                Command::none()
            }
        } else {
            Command::perform(async move {}, |_| Message::SetStage(Stage::Login))
        }
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let button = Button::new("Relays")
            .padding(10)
            .on_press(Message::Home(HomeMessage::GoToRelays));

        let content = Column::new()
            .push(button)
            .spacing(20)
            .padding(20)
            .max_width(600);

        Dashboard::new().view(ctx, content)
    }
}

impl From<HomeState> for Box<dyn State> {
    fn from(s: HomeState) -> Box<dyn State> {
        Box::new(s)
    }
}
