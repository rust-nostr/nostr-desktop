// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::alignment::Horizontal;
use iced::widget::{Button, Container, Row};
use iced::Length;

use crate::component::Icon;
use crate::message::Message;
use crate::stage::dashboard::component::post::TransparentStyle;
use crate::stage::dashboard::Stage;
use crate::theme::icon::{LOCK, NOTIFICATION};

pub struct Navbar;

impl Navbar {
    pub fn view<'a>() -> Container<'a, Message> {
        let content = Row::new()
            .push(
                Button::new(Icon::view(&NOTIFICATION))
                    .on_press(Message::SetDashboardStage(Stage::Notifications))
                    .style(TransparentStyle.into()),
            )
            .push(
                Button::new(Icon::view(&LOCK))
                    .on_press(Message::Lock)
                    .style(TransparentStyle.into()),
            )
            .spacing(15);
        Container::new(content)
            .width(Length::Fill)
            .padding(20)
            .center_y()
            .align_x(Horizontal::Right)
    }
}
