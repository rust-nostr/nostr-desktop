// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{column, row, text, Button, Column, Row, Text, TextInput};
use iced::{Command, Element};

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
    error: Option<String>,
}

impl RelaysState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.relay_url = String::new();
        self.error = None;
    }
}

impl State for RelaysState {
    fn title(&self) -> String {
        String::from("Nostr - Relays")
    }

    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message> {
        if let Some(client) = ctx.client.as_mut() {
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
        .padding(10)
        .size(20);

        let button = Button::new("Add")
            .padding(10)
            .on_press(Message::Menu(MenuMessage::Setting(SettingMessage::Relays(
                RelaysMessage::AddRelay,
            ))));

        let relays = if let Some(client) = ctx.client.as_ref() {
            let mut col_recipients = Column::new().push(Text::new("Relays:")).spacing(10);

            for (url, relay) in client.relays() {
                let button = Button::new("Remove")
                    .padding(10)
                    .style(iced::theme::Button::Destructive)
                    .on_press(Message::Menu(MenuMessage::Setting(SettingMessage::Relays(
                        RelaysMessage::RemoveRelay(url.to_string()),
                    ))));

                let status = relay.status_blocking();
                col_recipients = col_recipients.push(
                    Row::new()
                        .push(Text::new(url.to_string()))
                        .push(button)
                        .push(Text::new(status.to_string())),
                );
            }

            col_recipients
        } else {
            column![]
        };

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
