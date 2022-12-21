// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::theme;
use iced::widget::{button, Button, Container};
use iced::{Background, Length, Theme, Vector};

use crate::context::{Context, Stage};
use crate::theme::color::{PRIMARY, TRANSPARENT, WHITE};
use crate::Message;

pub const BUTTON_SIZE: u16 = 180;

pub struct ActiveStyle;

impl button::StyleSheet for ActiveStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector::default(),
            background: Some(Background::Color(PRIMARY)),
            border_radius: 10.0,
            border_width: 1.0,
            border_color: WHITE,
            text_color: WHITE,
        }
    }
}

impl From<ActiveStyle> for theme::Button {
    fn from(style: ActiveStyle) -> Self {
        theme::Button::Custom(Box::new(style))
    }
}

pub struct TransparentStyle;

impl button::StyleSheet for TransparentStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector::default(),
            background: Some(Background::Color(TRANSPARENT)),
            border_radius: 10.0,
            border_width: 1.0,
            border_color: WHITE,
            text_color: WHITE,
        }
    }
}

impl From<TransparentStyle> for theme::Button {
    fn from(style: TransparentStyle) -> Self {
        theme::Button::Custom(Box::new(style))
    }
}

#[derive(Debug, Clone, Default)]
pub struct SidebarButton<'a> {
    text: &'a str,
}

impl<'a> SidebarButton<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn view(&self, ctx: &Context, stage: Stage) -> Container<'a, Message> {
        let style = if ctx.stage.eq(&stage) {
            ActiveStyle.into()
        } else {
            TransparentStyle.into()
        };

        Container::new(
            Button::new(self.text)
                .padding(10)
                .on_press(Message::SetStage(stage))
                .width(Length::Units(BUTTON_SIZE))
                .style(style),
        )
    }
}
