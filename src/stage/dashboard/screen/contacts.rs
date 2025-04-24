// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::collections::HashMap;

use iced::widget::{image, Column, Row, Text};
use iced::{Command, Element};
use nostr_sdk::nostr::secp256k1::XOnlyPublicKey;

use crate::message::{DashboardMessage, Message};
use crate::stage::dashboard::component::{Contact, Dashboard};
use crate::stage::dashboard::{Context, State};

#[derive(Debug, Clone)]
pub enum ContactsMessage {
    SearchImage(XOnlyPublicKey, String),
    MaybeFoundImage(XOnlyPublicKey, Option<image::Handle>),
}

#[derive(Debug, Default)]
pub struct ContactsState {
    contacts: HashMap<XOnlyPublicKey, Contact>,
    error: Option<String>,
}

impl ContactsState {
    pub fn new() -> Self {
        Self {
            contacts: HashMap::new(),
            error: None,
        }
    }
}

impl State for ContactsState {
    fn title(&self) -> String {
        String::from("Nostr - Contacts")
    }

    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message> {
        let mut commands = Vec::new();

        if let Ok(store) = ctx.client.store() {
            for profile in store.get_contacts().unwrap_or_default().into_iter() {
                self.contacts
                    .entry(profile.pubkey)
                    .and_modify(|c| c.profile = profile.clone())
                    .or_insert_with(|| Contact::new(profile));
            }
        }

        if let Message::Dashboard(DashboardMessage::Contacts(msg)) = message {
            match msg {
                ContactsMessage::SearchImage(pk, url) => {
                    return Command::perform(async { fetch_image(url).await }, move |image| {
                        ContactsMessage::MaybeFoundImage(pk, image).into()
                    })
                }
                ContactsMessage::MaybeFoundImage(pk, image) => {
                    self.contacts.entry(pk).and_modify(|c| c.image = image);
                    return Command::perform(async {}, |_| Message::Tick);
                }
            }
        }

        for (pk, contact) in self.contacts.iter() {
            if contact.image.is_none() {
                if let Some(url) = contact.profile.picture.clone() {
                    let pk = *pk;
                    commands.push(Command::perform(async {}, move |_| {
                        ContactsMessage::SearchImage(pk, url).into()
                    }))
                }
            }
        }

        Command::batch(commands)
    }

    fn view(&self, ctx: &Context) -> Element<Message> {
        let mut contacts = Column::new().spacing(10);

        let mut contacts_vec: Vec<(&XOnlyPublicKey, &Contact)> = self.contacts.iter().collect();
        contacts_vec.sort_by(|a, b| a.1.profile.name.cmp(&b.1.profile.name));
        for (_, contact) in contacts_vec.iter() {
            contacts = contacts.push(Row::new().push(contact.view()));
        }

        let content = Column::new()
            .push(Row::new().push(if let Some(error) = &self.error {
                Row::new().push(Text::new(error))
            } else {
                Row::new()
            }))
            .push(contacts);

        Dashboard::new().view(ctx, content.spacing(20).padding(20))
    }
}

impl From<ContactsState> for Box<dyn State> {
    fn from(s: ContactsState) -> Box<dyn State> {
        Box::new(s)
    }
}

impl From<ContactsMessage> for Message {
    fn from(msg: ContactsMessage) -> Self {
        Self::Dashboard(DashboardMessage::Contacts(msg))
    }
}

pub async fn fetch_image(url: String) -> Option<image::Handle> {
    match reqwest::get(url).await {
        Ok(res) => match res.bytes().await {
            Ok(bytes) => return Some(image::Handle::from_memory(bytes.as_ref().to_vec())),
            Err(e) => log::error!("Impossible to fetch image bytes: {}", e),
        },
        Err(e) => log::error!("Impossible to fetch image: {}", e),
    }
    None
}
