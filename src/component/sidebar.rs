// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Button, Column, Container, Text};
use iced::Length;

use crate::context::{Menu, Setting, Stage};
use crate::{Context, Message};

#[derive(Debug, Clone, Default)]
pub struct Sidebar {}

impl Sidebar {
    pub fn view<'a>(&self, _ctx: &Context) -> Container<'a, Message> {
        let home_button = Button::new("Home").on_press(Message::SetStage(Stage::Menu(Menu::Home)));
        let explore_button = Button::new("Explore");
        let chat_button = Button::new("Chat");
        let contacts_button = Button::new("Contacts");
        let notifications_button = Button::new("Notifications");
        let profile_button = Button::new("Profile");
        let setting_button = Button::new("Setting")
            .on_press(Message::SetStage(Stage::Menu(Menu::Setting(Setting::Main))));

        let version = Text::new(format!(
            "{} v{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .size(16);

        sidebar(
            sidebar_menu(vec![
                Container::new(home_button.width(Length::Units(200))),
                Container::new(explore_button.width(Length::Units(200))),
                Container::new(chat_button.width(Length::Units(200))),
                Container::new(contacts_button.width(Length::Units(200))),
                Container::new(notifications_button.width(Length::Units(200))),
                Container::new(profile_button.width(Length::Units(200))),
                Container::new(setting_button.width(Length::Units(200))),
            ]),
            sidebar_menu(vec![Container::new(version)]),
        )
    }
}

pub fn sidebar<'a, T: 'a>(menu: Container<'a, T>, footer: Container<'a, T>) -> Container<'a, T> {
    Container::new(
        Column::new()
            .padding(10)
            .push(menu.height(Length::Fill))
            .push(footer.height(Length::Shrink)),
    )
}

pub fn sidebar_menu<'a, T: 'a>(items: Vec<Container<'a, T>>) -> Container<'a, T> {
    let mut col = Column::new().padding(15).spacing(15);
    for i in items {
        col = col.push(i)
    }
    Container::new(col)
}
