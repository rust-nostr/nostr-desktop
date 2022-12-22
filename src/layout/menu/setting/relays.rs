// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::collections::HashMap;
use std::time::Duration;

use iced::widget::{column, row, text, Button, Column, Row, Text, TextInput};
use iced::{time, Command, Element, Subscription};
use nostr_sdk::nostr::url::Url;
use nostr_sdk::Relay;

use super::SettingMessage;
use crate::component::Dashboard;
use crate::context::{Context, Stage};
use crate::layout::State;
use crate::message::{MenuMessage, Message};

#[derive(Debug, Clone)]
pub enum RelaysMessage {
    RelayUrlChanged(String),
    AddRelay,
    RemoveRelay(String),
}

#[derive(Debug, Default)]
pub struct RelaysState {
    relay_url: String,
    relays: HashMap<Url, Relay>,
    error: Option<String>,
}

impl RelaysState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.relay_url = String::new();
        self.relays = HashMap::new();
        self.error = None;
    }
}

impl State for RelaysState {
    fn title(&self) -> String {
        String::from("Nostr - Relays")
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            time::every(Duration::from_secs(5)).map(|_| Message::Tick)
        ])
    }

    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message> {
        if let Some(client) = ctx.client.clone() {
            if let Message::Menu(MenuMessage::Setting(SettingMessage::Relays(msg))) = message {
                match msg {
                    RelaysMessage::RelayUrlChanged(url) => self.relay_url = url,
                    RelaysMessage::AddRelay => match client.add_relay(&self.relay_url, None) {
                        Ok(_) => {
                            if let Err(e) = client.connect() {
                                self.error = Some(e.to_string())
                            } else {
                                self.relay_url.clear();
                                self.error = None;
                            }
                        }
                        Err(e) => self.error = Some(e.to_string()),
                    },
                    RelaysMessage::RemoveRelay(url) => match client.remove_relay(url) {
                        Ok(_) => self.error = None,
                        Err(e) => self.error = Some(e.to_string()),
                    },
                }
            }
            self.relays = client.relays();
            Command::none()
        } else {
            Command::perform(async move {}, |_| Message::SetStage(Stage::Login))
        }
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let heading = Text::new("Relays").size(30);

        let text_input = TextInput::new("Relay url", &self.relay_url, |s| {
            Message::Menu(MenuMessage::Setting(SettingMessage::Relays(
                RelaysMessage::RelayUrlChanged(s),
            )))
        })
        .on_submit(Message::Menu(MenuMessage::Setting(SettingMessage::Relays(
            RelaysMessage::AddRelay,
        ))))
        .padding(10)
        .size(20);

        let button = Button::new("Add")
            .padding(10)
            .on_press(Message::Menu(MenuMessage::Setting(SettingMessage::Relays(
                RelaysMessage::AddRelay,
            ))));

        let mut relays = Column::new().push(Text::new("Relays:")).spacing(10);

        for (url, relay) in self.relays.iter() {
            let button = Button::new("Remove")
                .padding(10)
                .style(iced::theme::Button::Destructive)
                .on_press(Message::Menu(MenuMessage::Setting(SettingMessage::Relays(
                    RelaysMessage::RemoveRelay(url.to_string()),
                ))));

            let status = relay.status_blocking();
            relays = relays.push(
                Row::new()
                    .push(Text::new(url.to_string()))
                    .push(button)
                    .push(Text::new(status.to_string())),
            );
        }

        let content = column![
            row![heading],
            row![text_input, button].spacing(10),
            if let Some(error) = &self.error {
                row![text(error)]
            } else {
                row![]
            },
            relays
        ];

        Dashboard::new().view(ctx, content)
    }
}

impl From<RelaysState> for Box<dyn State> {
    fn from(s: RelaysState) -> Box<dyn State> {
        Box::new(s)
    }
}
