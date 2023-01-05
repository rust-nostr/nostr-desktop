// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{button, Button, Container, Row, Text};
use iced::{theme, Alignment, Background, Length, Theme, Vector};

use crate::message::Message;
use crate::stage::dashboard::{Context, Stage};
use crate::theme::color::{PRIMARY, TRANSPARENT, WHITE};

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

#[derive(Clone)]
pub struct SidebarButton<'a> {
    text: &'a str,
    icon: Text<'a>,
}

impl<'a> SidebarButton<'a> {
    pub fn new(text: &'a str, icon: Text<'a>) -> Self {
        Self { text, icon }
    }

    pub fn view(&self, ctx: &Context, stage: Stage) -> Container<'a, Message> {
        let style = if ctx.stage.eq(&stage) {
            ActiveStyle.into()
        } else {
            TransparentStyle.into()
        };

        let content = Container::new(
            Row::new()
                .push(self.icon.clone())
                .push(Text::new(self.text))
                .spacing(10)
                .width(iced::Length::Fill)
                .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .center_x()
        .padding(5);

        Container::new(
            Button::new(content)
                .on_press(Message::SetDashboardStage(stage))
                .width(Length::Units(BUTTON_SIZE))
                .style(style),
        )
    }

    pub fn _view(&self, _ctx: &Context, msg: Message) -> Container<'a, Message> {
        let content = Container::new(
            Row::new()
                .push(self.icon.clone())
                .push(Text::new(self.text))
                .spacing(10)
                .width(iced::Length::Fill)
                .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .center_x()
        .padding(5);

        Container::new(
            Button::new(content)
                .on_press(msg)
                .width(Length::Units(BUTTON_SIZE))
                .style(TransparentStyle.into()),
        )
    }
}
