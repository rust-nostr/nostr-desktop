// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Column, Container, Row};
use iced::{Element, Length};

use crate::{Context, Message};

use super::sidebar::Sidebar;

#[derive(Debug, Clone, Default)]
pub struct Dashboard {
    sidebar: Sidebar,
}

impl Dashboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view<'a, T>(&self, ctx: &Context, content: T) -> Element<'a, Message>
    where
        T: Into<Element<'a, Message>>,
    {
        Column::new()
            .push(
                Row::new()
                    .push(
                        self.sidebar
                            .view(ctx)
                            .width(Length::Shrink)
                            .height(Length::Fill),
                    )
                    .push(
                        Container::new(content)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center_x(),
                    ),
            )
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}
