// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Column, Container, Row, Rule, Scrollable};
use iced::{Element, Length};

use crate::stage::dashboard::Context;
use crate::Message;

mod navbar;
mod sidebar;

use self::navbar::Navbar;
use self::sidebar::Sidebar;

#[derive(Clone, Default)]
pub struct Dashboard;

impl Dashboard {
    pub fn new() -> Self {
        Self
    }

    pub fn view<'a, T>(&self, ctx: &Context, content: T) -> Element<'a, Message>
    where
        T: Into<Element<'a, Message>>,
    {
        Column::new()
            .push(
                Row::new()
                    .push(
                        Sidebar::new()
                            .view(ctx)
                            .width(Length::Shrink)
                            .height(Length::Fill),
                    )
                    .push(Rule::vertical(1))
                    .push(
                        Column::new()
                            .push(Navbar::view())
                            .push(Rule::horizontal(1))
                            .push(
                                Container::new(
                                    Scrollable::new(content).on_scroll(Message::Scrolled),
                                )
                                //.max_width(600)
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .center_x(),
                            ),
                    ),
            )
            //.max_width(1200)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}
