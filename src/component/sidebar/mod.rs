// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Column, Container, Text};
use iced::Length;

mod button;

use self::button::{SidebarButton, BUTTON_SIZE};
use crate::context::{Menu, Setting, Stage};
use crate::{Context, Message};

#[derive(Debug, Clone, Default)]
pub struct Sidebar;

impl Sidebar {
    pub fn view<'a>(&self, ctx: &Context) -> Container<'a, Message> {
        let home_button = SidebarButton::new("Home").view(ctx, Stage::Menu(Menu::Home));
        let explore_button = SidebarButton::new("Explore").view(ctx, Stage::Menu(Menu::Explore));
        let chat_button = SidebarButton::new("Chats").view(ctx, Stage::Menu(Menu::Chats));
        let contacts_button = SidebarButton::new("Contacts").view(ctx, Stage::Menu(Menu::Contacts));
        let notifications_button =
            SidebarButton::new("Notifications").view(ctx, Stage::Menu(Menu::Notifications));
        let profile_button = SidebarButton::new("Profile").view(ctx, Stage::Menu(Menu::Profile));
        let setting_button =
            SidebarButton::new("Settings").view(ctx, Stage::Menu(Menu::Setting(Setting::Main)));

        let version = Text::new(format!(
            "{} v{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .size(16);

        sidebar(
            sidebar_menu(vec![
                home_button,
                explore_button,
                chat_button,
                contacts_button,
                notifications_button,
                profile_button,
                setting_button,
            ]),
            sidebar_menu(vec![Container::new(version)
                .width(Length::Units(BUTTON_SIZE))
                .center_x()]),
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
