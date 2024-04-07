// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::alignment::Horizontal;
use iced::widget::Text;
use iced::{Font, Length};

pub struct Icon;

impl Icon {
    pub fn view(unicode: &'static char) -> Text<'static> {
        Text::new(unicode.to_string())
            .font(Font::with_name("bootstrap-icons"))
            .width(Length::Fixed(20.0))
            .horizontal_alignment(Horizontal::Center)
            .size(20)
    }
}
