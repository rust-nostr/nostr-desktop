// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

use iced::widget::{text, Button, Checkbox, Column, Row, Text, TextInput};
use iced::{time, Alignment, Command, Element, Length, Subscription};
use nostr_sdk::nostr::url::Url;
use nostr_sdk::Client;
use nostr_sdk::{Relay, RelayStatus};

use super::SettingMessage;
use crate::component::{Circle, Icon};
use crate::message::{DashboardMessage, Message};
use crate::stage::dashboard::component::Dashboard;
use crate::stage::dashboard::{Context, State};
use crate::theme::color::{GREEN, RED, YELLOW};
use crate::theme::icon::TRASH;
use crate::RUNTIME;

#[derive(Debug, Clone)]
pub enum RelaysMessage {
    RelayUrlChanged(String),
    ProxyChanged(String),
    ProxyToggled(bool),
    AddRelay,
    RemoveRelay(String),
    SetRelays(HashMap<Url, (Relay, RelayStatus)>),
}

#[derive(Debug, Default)]
pub struct RelaysState {
    relay_url: String,
    proxy: String,
    use_proxy: bool,
    relays: HashMap<Url, (Relay, RelayStatus)>,
    error: Option<String>,
}

impl RelaysState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.relay_url = String::new();
        self.proxy = String::new();
        self.use_proxy = false;
        self.relays = HashMap::new();
        self.error = None;
    }

    async fn add_relay(&mut self, client: &Client, proxy: Option<SocketAddr>) {
        match client.add_relay(&self.relay_url, proxy).await {
            Ok(_) => {
                if let Err(e) = client.connect().await {
                    self.error = Some(e.to_string())
                } else {
                    self.relay_url.clear();
                    self.error = None;
                }
            }
            Err(e) => self.error = Some(e.to_string()),
        }
    }
}

impl State for RelaysState {
    fn title(&self) -> String {
        String::from("Nostr - Relays")
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            time::every(Duration::from_secs(7)).map(|_| Message::Tick)
        ])
    }

    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message> {
        let client = ctx.client.clone();
        if let Message::Dashboard(DashboardMessage::Setting(SettingMessage::Relays(msg))) = message
        {
            match msg {
                RelaysMessage::RelayUrlChanged(url) => self.relay_url = url,
                RelaysMessage::ProxyChanged(proxy) => self.proxy = proxy,
                RelaysMessage::ProxyToggled(value) => self.use_proxy = value,
                RelaysMessage::AddRelay => {
                    if self.use_proxy {
                        match self.proxy.parse() {
                            Ok(proxy) => RUNTIME
                                .block_on(async { self.add_relay(&client, Some(proxy)).await }),
                            Err(e) => self.error = Some(e.to_string()),
                        }
                    } else {
                        RUNTIME.block_on(async { self.add_relay(&client, None).await });
                    };
                }
                RelaysMessage::RemoveRelay(url) => RUNTIME.block_on(async {
                    match client.remove_relay(url).await {
                        Ok(_) => self.error = None,
                        Err(e) => self.error = Some(e.to_string()),
                    }
                }),
                RelaysMessage::SetRelays(relays) => self.relays = relays,
            }
        }
        Command::perform(
            async move {
                let mut relays = HashMap::new();
                for (url, relay) in client.relays().await.into_iter() {
                    relays.insert(url, (relay.clone(), relay.status().await));
                }
                relays
            },
            |relays| {
                Message::Dashboard(DashboardMessage::Setting(SettingMessage::Relays(
                    RelaysMessage::SetRelays(relays),
                )))
            },
        )
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let heading = Text::new("Relays").size(30);

        let on_submit = Message::Dashboard(DashboardMessage::Setting(SettingMessage::Relays(
            RelaysMessage::AddRelay,
        )));

        let relay_url_input = TextInput::new("Relay url", &self.relay_url, |s| {
            Message::Dashboard(DashboardMessage::Setting(SettingMessage::Relays(
                RelaysMessage::RelayUrlChanged(s),
            )))
        })
        .on_submit(on_submit.clone())
        .padding(10)
        .size(20);

        let use_proxy_checkbox = Checkbox::new(self.use_proxy, "Use proxy", |value| {
            Message::Dashboard(DashboardMessage::Setting(SettingMessage::Relays(
                RelaysMessage::ProxyToggled(value),
            )))
        });

        let proxy_input = TextInput::new("Socks5 proxy (ex. 127.0.0.1:9050)", &self.proxy, |s| {
            Message::Dashboard(DashboardMessage::Setting(SettingMessage::Relays(
                RelaysMessage::ProxyChanged(s),
            )))
        })
        .on_submit(on_submit.clone())
        .padding(10)
        .size(20);

        let button = Button::new("Add").padding(10).on_press(on_submit);

        let mut relays = Column::new().spacing(10);

        if !self.relays.is_empty() {
            relays = relays.push(Text::new("Relays:"));
        }

        for (url, (relay, status)) in self.relays.iter() {
            let button = Button::new(Icon::view(&TRASH))
                .padding(10)
                .style(iced::theme::Button::Destructive)
                .on_press(Message::Dashboard(DashboardMessage::Setting(
                    SettingMessage::Relays(RelaysMessage::RemoveRelay(url.to_string())),
                )));

            let status = match status {
                RelayStatus::Connected => Circle::new(7.0).color(GREEN),
                RelayStatus::Initialized | RelayStatus::Connecting => {
                    Circle::new(7.0).color(YELLOW)
                }
                RelayStatus::Disconnected | RelayStatus::Terminated => Circle::new(7.0).color(RED),
            };

            let info = Row::new()
                .push(status)
                .push(Text::new(url.to_string()))
                .push(Text::new(format!("Proxy: {}", relay.proxy().is_some())))
                .spacing(20)
                .align_items(Alignment::Center)
                .width(Length::Fill);

            relays = relays.push(
                Row::new()
                    .push(info)
                    .push(button)
                    .spacing(20)
                    .align_items(Alignment::Center),
            );
        }

        let content = Column::new()
            .push(Row::new().push(heading))
            .push(Row::new().push(relay_url_input).push(button).spacing(10))
            .push(if self.use_proxy {
                Column::new()
                    .push(Row::new().push(proxy_input))
                    .push(Row::new().push(use_proxy_checkbox))
                    .spacing(20)
            } else {
                Column::new().push(Row::new().push(use_proxy_checkbox))
            })
            .push(if let Some(error) = &self.error {
                Row::new().push(text(error))
            } else {
                Row::new()
            })
            .push(relays);

        Dashboard::new().view(ctx, content)
    }
}

impl From<RelaysState> for Box<dyn State> {
    fn from(s: RelaysState) -> Box<dyn State> {
        Box::new(s)
    }
}
