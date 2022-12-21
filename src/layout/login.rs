// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::str::FromStr;

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Command, Element, Length};
use nostr_sdk::client::blocking::Client;
use nostr_sdk::nostr::key::{FromBech32, Keys};

use crate::context::{Context, Menu, Stage};
use crate::layout::State;
use crate::Message;

#[derive(Debug, Clone)]
pub enum LoginMessage {
    SecretKeyChanged(String),
    ButtonPressed,
}

#[derive(Debug, Default)]
pub struct LoginState {
    secret_key: String,
    error: Option<String>,
}

impl LoginState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.secret_key = String::new();
        self.error = None;
    }
}

impl State for LoginState {
    fn title(&self) -> String {
        String::from("Nostr - Login")
    }

    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message> {
        if let Message::Login(msg) = message {
            match msg {
                LoginMessage::SecretKeyChanged(secret_key) => self.secret_key = secret_key,
                LoginMessage::ButtonPressed => match Keys::from_str(&self.secret_key) {
                    Ok(keys) => {
                        self.clear();
                        ctx.set_client(Some(Client::new(&keys)));
                        return Command::perform(async move {}, |_| {
                            Message::SetStage(Stage::Menu(Menu::Home))
                        });
                    }
                    Err(_) => match Keys::from_bech32(&self.secret_key) {
                        Ok(keys) => {
                            self.clear();
                            ctx.set_client(Some(Client::new(&keys)));
                            return Command::perform(async move {}, |_| {
                                Message::SetStage(Stage::Menu(Menu::Home))
                            });
                        }
                        Err(_) => self.error = Some("Invalid secret key".to_string()),
                    },
                },
            }
        };

        Command::none()
    }

    fn view(&self, _ctx: &Context) -> Element<Message> {
        let text_input = text_input("Secret key", &self.secret_key, |s| {
            Message::Login(LoginMessage::SecretKeyChanged(s))
        })
        .padding(10)
        .size(20);

        let button = button("Login")
            .padding(10)
            .on_press(Message::Login(LoginMessage::ButtonPressed));

        let content = column![
            row![text_input, button].spacing(10),
            if let Some(error) = &self.error {
                row![text(error)]
            } else {
                row![]
            }
        ]
        .spacing(20)
        .padding(20)
        .max_width(600);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl From<LoginState> for Box<dyn State> {
    fn from(s: LoginState) -> Box<dyn State> {
        Box::new(s)
    }
}
