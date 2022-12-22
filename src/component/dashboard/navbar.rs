// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Column, Container, Row};
use iced::Length;

pub struct Navbar;

impl Navbar {
    pub fn view<'a, T: 'a>() -> Container<'a, T> {
        let content = Row::new().push(Column::new().width(Length::Units(10)));
        Container::new(content)
            .width(Length::Fill)
            .padding(10)
            .center_y()
    }
}
