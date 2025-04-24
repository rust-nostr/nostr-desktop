// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::net::SocketAddr;
use std::time::Duration;

use iced::widget::{text, Button, Checkbox, Column, Row, Rule, Text, TextInput};
use iced::{time, Alignment, Command, Element, Length, Subscription};
use nostr_sdk::nostr::url::Url;
use nostr_sdk::RelayStatus;

use super::SettingMessage;
use crate::component::{Circle, Icon};
use crate::message::{DashboardMessage, Message};
use crate::stage::dashboard::component::Dashboard;
use crate::stage::dashboard::{Context, State};
use crate::theme::color::{GREEN, GREY, RED, YELLOW};
use crate::theme::icon::TRASH;
use crate::RUNTIME;

#[derive(Debug, Clone)]
pub enum RelaysMessage {
    RelayUrlChanged(String),
    ProxyChanged(String),
    ProxyToggled(bool),
    AddRelay,
    AddRelayFromStore(Url, Option<SocketAddr>),
    RemoveRelay(String),
    DisconnectRelay(String),
    UpdateRelays,
    SetRelays(Vec<(RelayStatus, Url, Option<SocketAddr>)>),
}

#[derive(Debug, Default)]
pub struct RelaysState {
    loaded: bool,
    relay_url: String,
    proxy: String,
    use_proxy: bool,
    relays: Vec<(RelayStatus, Url, Option<SocketAddr>)>,
    error: Option<String>,
}

impl RelaysState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.loaded = false;
        self.relay_url = String::new();
        self.proxy = String::new();
        self.use_proxy = false;
        self.relays = Vec::new();
        self.error = None;
    }

    async fn add_relay(&mut self, ctx: &Context, proxy: Option<SocketAddr>) {
        match ctx.client.add_relay(&self.relay_url, proxy).await {
            Ok(_) => {
                ctx.client.connect().await;
                self.relay_url.clear();
                self.error = None;
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
            time::every(Duration::from_secs(10)).map(|_| RelaysMessage::UpdateRelays.into())
        ])
    }

    fn load(&mut self, _ctx: &Context) -> Command<Message> {
        self.loaded = true;
        Command::perform(async {}, |_| RelaysMessage::UpdateRelays.into())
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
                            Ok(proxy) => {
                                RUNTIME.block_on(async { self.add_relay(ctx, Some(proxy)).await })
                            }
                            Err(e) => self.error = Some(e.to_string()),
                        }
                    } else {
                        RUNTIME.block_on(async { self.add_relay(ctx, None).await });
                    };
                    return self.load(ctx);
                }
                RelaysMessage::RemoveRelay(url) => {
                    return Command::perform(
                        async move {
                            if let Err(e) = client.remove_relay(&url).await {
                                log::error!("Impossible to remove {}: {}", url, e);
                            }
                        },
                        |_| RelaysMessage::UpdateRelays.into(),
                    )
                }
                RelaysMessage::AddRelayFromStore(url, proxy) => {
                    return Command::perform(
                        async move {
                            if let Err(e) = client.add_relay(url.clone(), proxy).await {
                                log::error!("Impossible to add {}: {}", url, e);
                            }
                            client.connect().await;
                        },
                        |_| RelaysMessage::UpdateRelays.into(),
                    )
                }
                RelaysMessage::DisconnectRelay(url) => {
                    return Command::perform(
                        async move {
                            if let Err(e) = client.disconnect_relay(&url).await {
                                log::error!("Impossible to disconnect {}: {}", url, e);
                            }
                        },
                        |_| RelaysMessage::UpdateRelays.into(),
                    )
                }
                RelaysMessage::SetRelays(relays) => self.relays = relays,
                RelaysMessage::UpdateRelays => {
                    return Command::perform(
                        async move {
                            let mut relays = Vec::new();
                            for (url, relay) in client.relays().await.into_iter() {
                                relays.push((relay.status().await, url, relay.proxy()));
                            }
                            relays
                        },
                        |relays| RelaysMessage::SetRelays(relays).into(),
                    )
                }
            }
        }

        if self.loaded {
            Command::none()
        } else {
            self.load(ctx)
        }
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let heading = Text::new("Relays").size(30);

        let on_submit = Message::Dashboard(DashboardMessage::Setting(SettingMessage::Relays(
            RelaysMessage::AddRelay,
        )));

        let relay_url_input = TextInput::new("Relay url", &self.relay_url)
            .on_input(|s| {
                Message::Dashboard(DashboardMessage::Setting(SettingMessage::Relays(
                    RelaysMessage::RelayUrlChanged(s),
                )))
            })
            .on_submit(on_submit.clone())
            .padding(10)
            .size(20);

        let use_proxy_checkbox = Checkbox::new("Use proxy", self.use_proxy).on_toggle(|value| {
            Message::Dashboard(DashboardMessage::Setting(SettingMessage::Relays(
                RelaysMessage::ProxyToggled(value),
            )))
        });

        let proxy_input = TextInput::new("Socks5 proxy (ex. 127.0.0.1:9050)", &self.proxy)
            .on_input(|s| {
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

        for (status, url, proxy) in self.relays.iter() {
            let status = match status {
                RelayStatus::Initialized => Circle::new(7.0).color(GREY),
                RelayStatus::Connecting => Circle::new(7.0).color(YELLOW),
                RelayStatus::Connected => Circle::new(7.0).color(GREEN),
                RelayStatus::Disconnected => Circle::new(7.0).color(RED),
                RelayStatus::Terminated => continue,
            };

            let button = Button::new(Text::new("Disconnect"))
                .padding(10)
                .style(iced::theme::Button::Destructive)
                .on_press(Message::Dashboard(DashboardMessage::Setting(
                    SettingMessage::Relays(RelaysMessage::DisconnectRelay(url.to_string())),
                )));

            let button_remove = Button::new(Icon::view(&TRASH))
                .padding(10)
                .style(iced::theme::Button::Destructive)
                .on_press(Message::Dashboard(DashboardMessage::Setting(
                    SettingMessage::Relays(RelaysMessage::RemoveRelay(url.to_string())),
                )));

            let info = Row::new()
                .push(status)
                .push(Text::new(url.to_string()))
                .push(Text::new(format!("Proxy: {}", proxy.is_some())))
                .spacing(20)
                .align_items(Alignment::Center)
                .width(Length::Fill);

            relays = relays.push(
                Row::new()
                    .push(info)
                    .push(button)
                    .push(button_remove)
                    .spacing(20)
                    .align_items(Alignment::Center),
            );
        }

        let mut saved_relays = Column::new().spacing(10);

        if let Ok(store) = ctx.client.store() {
            let relays = store.get_relays(false).unwrap_or_default();
            if !relays.is_empty() {
                saved_relays = saved_relays
                    .push(Rule::horizontal(1))
                    .push(Text::new("Saved relays"));
            }

            for (url, proxy) in relays.into_iter() {
                let button_connect =
                    Button::new(Text::new("Add"))
                        .padding(10)
                        .on_press(Message::Dashboard(DashboardMessage::Setting(
                            SettingMessage::Relays(RelaysMessage::AddRelayFromStore(
                                url.clone(),
                                proxy,
                            )),
                        )));
                let button_remove = Button::new(Icon::view(&TRASH))
                    .padding(10)
                    .style(iced::theme::Button::Destructive)
                    .on_press(Message::Dashboard(DashboardMessage::Setting(
                        SettingMessage::Relays(RelaysMessage::RemoveRelay(url.to_string())),
                    )));

                let info = Row::new()
                    .push(Text::new(url.to_string()))
                    .push(Text::new(format!("Proxy: {}", proxy.is_some())))
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .width(Length::Fill);

                saved_relays = saved_relays.push(
                    Row::new()
                        .push(info)
                        .push(button_connect)
                        .push(button_remove)
                        .spacing(20)
                        .align_items(Alignment::Center),
                );
            }
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
            .push(relays)
            .push(saved_relays);

        Dashboard::new().view(ctx, content.spacing(20).padding([20, 30]))
    }
}

impl From<RelaysState> for Box<dyn State> {
    fn from(s: RelaysState) -> Box<dyn State> {
        Box::new(s)
    }
}

impl From<RelaysMessage> for Message {
    fn from(msg: RelaysMessage) -> Self {
        Self::Dashboard(DashboardMessage::Setting(SettingMessage::Relays(msg)))
    }
}
