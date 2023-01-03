// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::time::Duration;

use async_stream::stream;
use iced::Subscription;
use iced_futures::BoxStream;
use nostr_sdk::nostr::secp256k1::XOnlyPublicKey;
use nostr_sdk::nostr::{Contact, Event, Kind, KindBase, Metadata, SubscriptionFilter, Tag};
use nostr_sdk::Client;
use nostr_sdk::RelayPoolNotifications;
use tokio::sync::mpsc;

use crate::nostr::db::Store;
use crate::RUNTIME;

use super::db::model::Profile;
use super::filters::Filters;

pub struct NostrSync {
    client: Client,
    store: Store,
    join: Option<tokio::task::JoinHandle<()>>,
}

impl<H, I> iced::subscription::Recipe<H, I> for NostrSync
where
    H: std::hash::Hasher,
{
    type Output = Event;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(mut self: Box<Self>, _input: BoxStream<I>) -> BoxStream<Self::Output> {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let client = self.client.clone();
        let store = self.store.clone();

        super::thread::spawn("filters", move || {
            let my_public_key = client.keys().public_key();

            let mut filters: Filters = Filters {
                contact_list: SubscriptionFilter::new()
                    .author(my_public_key)
                    .kind(Kind::Base(KindBase::ContactList))
                    .limit(1),
                encrypted_dm: SubscriptionFilter::new()
                    .pubkey(my_public_key)
                    .kind(Kind::Base(KindBase::EncryptedDirectMessage)),
                following_authors: SubscriptionFilter::new().author(my_public_key).kinds(vec![
                    Kind::Base(KindBase::Metadata),
                    Kind::Base(KindBase::TextNote),
                    Kind::Base(KindBase::Boost),
                    Kind::Base(KindBase::Reaction),
                ]),
            };

            if let Ok(mut authors) = store.get_authors() {
                if !authors.contains(&my_public_key) {
                    authors.push(my_public_key);
                }
                filters.following_authors = SubscriptionFilter::new().authors(authors).kinds(vec![
                    Kind::Base(KindBase::Metadata),
                    Kind::Base(KindBase::TextNote),
                    Kind::Base(KindBase::Boost),
                    Kind::Base(KindBase::Reaction),
                ]);
            }

            if let Err(e) = RUNTIME.block_on(async { client.subscribe(filters.to_vec()).await }) {
                log::error!("Impossible to subscribe to events: {}", e.to_string());
            }

            loop {
                std::thread::sleep(Duration::from_secs(30));

                if let Ok(mut authors) = store.get_authors() {
                    if !authors.contains(&my_public_key) {
                        authors.push(my_public_key);
                    }
                    if filters.following_authors.authors.as_ref() != Some(&authors) {
                        filters.following_authors =
                            SubscriptionFilter::new().authors(authors).kinds(vec![
                                Kind::Base(KindBase::Metadata),
                                Kind::Base(KindBase::TextNote),
                                Kind::Base(KindBase::Boost),
                                Kind::Base(KindBase::Reaction),
                            ]);

                        if let Err(e) =
                            RUNTIME.block_on(async { client.subscribe(filters.to_vec()).await })
                        {
                            log::error!("Impossible to subscribe to events: {}", e.to_string());
                        }
                    }
                }
            }
        });

        let client = self.client.clone();
        let store = self.store.clone();
        let join = tokio::task::spawn(async move {
            loop {
                let my_public_key = client.keys().public_key();
                let mut notifications = client.notifications();
                while let Ok(notification) = notifications.recv().await {
                    match notification {
                        RelayPoolNotifications::ReceivedEvent(event) => {
                            process_event(my_public_key, &store, &event);
                            sender.send(event).ok();
                        }
                        RelayPoolNotifications::ReceivedMessage(_msg) => {}
                    }
                }
            }
        });
        self.join = Some(join);
        let stream = stream! {
            while let Some(item) = receiver.recv().await {
                yield item;
            }
        };
        Box::pin(stream)
    }
}

impl NostrSync {
    pub fn subscription(client: Client, store: Store) -> Subscription<Event> {
        Subscription::from_recipe(Self {
            client,
            store,
            join: None,
        })
    }
}

fn process_event(my_public_key: XOnlyPublicKey, store: &Store, event: &Event) {
    match event.kind {
        Kind::Base(KindBase::Metadata) => {
            if let Ok(profile) = store.get_profile(event.pubkey) {
                if event.created_at > profile.metadata_at {
                    if let Ok(metadata) = Metadata::from_json(&event.content) {
                        if let Err(e) = store.update_profile(Profile {
                            pubkey: event.pubkey,
                            name: metadata.name,
                            display_name: metadata.display_name,
                            about: metadata.about,
                            website: metadata.website.map(|u| u.to_string()),
                            picture: metadata.picture.map(|u| u.to_string()),
                            nip05: metadata.nip05,
                            lud06: metadata.lud06,
                            lud16: metadata.lud16,
                            is_contact: profile.is_contact,
                            metadata_at: event.created_at,
                        }) {
                            log::error!("Impossible to update profile: {}", e.to_string());
                        }
                    }
                }
            } else if let Ok(metadata) = Metadata::from_json(&event.content) {
                if let Err(e) = store.insert_profile(Profile {
                    pubkey: event.pubkey,
                    name: metadata.name,
                    display_name: metadata.display_name,
                    about: metadata.about,
                    website: metadata.website.map(|u| u.to_string()),
                    picture: metadata.picture.map(|u| u.to_string()),
                    nip05: metadata.nip05,
                    lud06: metadata.lud06,
                    lud16: metadata.lud16,
                    is_contact: event.pubkey == my_public_key,
                    metadata_at: event.created_at,
                }) {
                    log::error!("Impossible to insert profile: {}", e.to_string());
                }
            }
        }
        Kind::Base(KindBase::ContactList) => {
            let mut contact_list: Vec<Contact> = Vec::new();
            for tag in event.tags.clone().into_iter() {
                match tag {
                    Tag::PubKey(pk, relay_url) => {
                        contact_list.push(Contact::new::<String>(pk, relay_url, None))
                    }
                    Tag::ContactList {
                        pk,
                        relay_url,
                        alias,
                    } => contact_list.push(Contact::new(pk, relay_url, alias)),
                    _ => (),
                }
            }
            if let Err(e) = store.set_contacts(contact_list) {
                log::error!("Impossible to save contact list: {}", e.to_string());
            }
        }
        _ => {
            if let Err(e) = store.save_event(event) {
                log::error!("Impossible to save text note: {}", e.to_string());
            }
        }
    };
}
