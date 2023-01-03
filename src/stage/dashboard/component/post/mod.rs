// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use chrono::{DateTime, NaiveDateTime, Utc};
use iced::widget::{button, Button, Column, Container, Row, Rule, Space, Text};
use iced::{theme, Background, Length, Theme, Vector};
use nostr_sdk::nostr::Event;

use crate::component::Icon;
use crate::message::Message;
use crate::stage::dashboard::Context;
use crate::theme::color::{TRANSPARENT, WHITE};
use crate::theme::icon::{CHAT, HEART, REPEAT};

pub struct TransparentStyle;

impl button::StyleSheet for TransparentStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector::default(),
            background: Some(Background::Color(TRANSPARENT)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: TRANSPARENT,
            text_color: WHITE,
        }
    }
}

impl From<TransparentStyle> for theme::Button {
    fn from(style: TransparentStyle) -> Self {
        theme::Button::Custom(Box::new(style))
    }
}

pub struct Post {
    event: Event,
}

impl Post {
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    pub fn view<'a>(&self, ctx: &Context) -> Container<'a, Message> {
        let display_name = if let Ok(profile) = ctx.store.get_profile(self.event.pubkey) {
            profile.display_name.unwrap_or_else(|| {
                let pk = self.event.pubkey.to_string();
                format!("{}:{}", &pk[0..8], &pk[pk.len() - 8..])
            })
        } else {
            let pk = self.event.pubkey.to_string();
            format!("{}:{}", &pk[0..8], &pk[pk.len() - 8..])
        };

        let buttons = Row::new()
            .push(Button::new(Icon::view(&CHAT).size(18)).style(TransparentStyle.into()))
            .push(Button::new(Icon::view(&REPEAT).size(18)).style(TransparentStyle.into()))
            .push(Button::new(Icon::view(&HEART).size(18)).style(TransparentStyle.into()))
            .spacing(20);

        let ndt = NaiveDateTime::from_timestamp_opt(self.event.created_at as i64, 0)
            .unwrap_or(NaiveDateTime::MIN);
        let dt: DateTime<Utc> = DateTime::from_utc(ndt, Utc);

        let post = Column::new()
            .push(Row::new().push(Text::new(display_name)))
            .push(Row::new().push(Text::new(self.event.content.clone())))
            .push(Space::with_height(Length::Units(15)))
            .push(Row::new().push(Text::new(dt.format("%Y-%m-%d %H:%M:%S").to_string()).size(14)))
            .push(buttons)
            .push(Rule::horizontal(1))
            .spacing(10);

        Container::new(post).padding(15)
    }
}
