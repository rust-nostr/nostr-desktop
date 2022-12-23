// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Column, Row, Text};
use iced::{Command, Element};
use nostr_sdk::nostr::Contact;

use crate::message::{DashboardMessage, Message};
use crate::stage::dashboard::component::Dashboard;
use crate::stage::dashboard::{Context, State};

#[derive(Debug, Clone)]
pub enum ContactsMessage {}

#[derive(Debug, Default)]
pub struct ContactsState {
    contacts: Vec<Contact>,
    error: Option<String>,
}

impl ContactsState {
    pub fn new() -> Self {
        Self {
            contacts: Vec::new(),
            error: None,
        }
    }
}

impl State for ContactsState {
    fn title(&self) -> String {
        String::from("Nostr - Contacts")
    }

    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message> {
        if let Ok(contacts) = ctx.store.get_contacts() {
            self.contacts = contacts;
        } else {
            self.error = Some("Impossible to get contacts".to_string());
        }

        if let Message::Dashboard(DashboardMessage::Contacts(_msg)) = message {
            Command::none()
        } else {
            Command::none()
        }
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let mut contacts = Column::new().spacing(10);

        for contact in self.contacts.iter() {
            if let Ok(profile) = ctx.store.get_profile(contact.pk) {
                let metadata = profile.metadata;
                let mut row = Row::new();

                if let Some(name) = metadata.name {
                    row = row.push(Text::new(name));
                }

                if let Some(display_name) = metadata.display_name {
                    row = row.push(Text::new(display_name));
                }

                if let Some(picture) = metadata.picture {
                    row = row.push(Text::new(picture.to_string()));
                }

                contacts = contacts.push(row);
            } else {
                contacts = contacts.push(Row::new().push(Text::new(contact.pk.to_string())));
            }
        }

        let content = Column::new()
            .push(Row::new().push(if let Some(error) = &self.error {
                Row::new().push(Text::new(error))
            } else {
                Row::new()
            }))
            .push(contacts);

        Dashboard::new().view(ctx, content)
    }
}

impl From<ContactsState> for Box<dyn State> {
    fn from(s: ContactsState) -> Box<dyn State> {
        Box::new(s)
    }
}