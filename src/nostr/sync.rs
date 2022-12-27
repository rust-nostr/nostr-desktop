// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::str::FromStr;
use std::time::Duration;

use async_stream::stream;
use iced::Subscription;
use iced_futures::BoxStream;
use nostr_sdk::nostr::secp256k1::XOnlyPublicKey;
use nostr_sdk::nostr::{Contact, Event, Kind, KindBase, Metadata, SubscriptionFilter};
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
            let mut filters: Filters = Filters {
                contact_list: SubscriptionFilter::new()
                    .author(client.keys().public_key())
                    .kind(Kind::Base(KindBase::ContactList))
                    .limit(1),
                encrypted_dm: SubscriptionFilter::new()
                    .pubkey(client.keys().public_key())
                    .kind(Kind::Base(KindBase::EncryptedDirectMessage)),
                following_authors: SubscriptionFilter::new(),
            };

            if let Ok(authors) = store.get_authors() {
                filters.following_authors = SubscriptionFilter::new().authors(authors).kinds(vec![
                    Kind::Base(KindBase::Metadata),
                    Kind::Base(KindBase::TextNote),
                ]);
            }

            if let Err(e) = RUNTIME.block_on(async { client.subscribe(filters.to_vec()).await }) {
                log::error!("Impossible to subscribe to events: {}", e.to_string());
            }

            loop {
                std::thread::sleep(Duration::from_secs(30));

                if let Ok(authors) = store.get_authors() {
                    if filters.following_authors.authors.as_ref() != Some(&authors) {
                        filters.following_authors =
                            SubscriptionFilter::new().authors(authors).kinds(vec![
                                Kind::Base(KindBase::Metadata),
                                Kind::Base(KindBase::TextNote),
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
                let mut notifications = client.notifications();
                while let Ok(notification) = notifications.recv().await {
                    match notification {
                        RelayPoolNotifications::ReceivedEvent(event) => {
                            process_event(&store, &event);
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

fn process_event(store: &Store, event: &Event) {
    let mut authors: Vec<XOnlyPublicKey> = vec![event.pubkey];

    match event.kind {
        Kind::Base(KindBase::Metadata) => {
            if let Ok(profile) = store.get_profile(event.pubkey) {
                if event.created_at > profile.timestamp {
                    if let Ok(metadata) = Metadata::from_json(&event.content) {
                        if let Err(e) = store.set_profile(
                            event.pubkey,
                            Profile {
                                metadata,
                                timestamp: event.created_at,
                            },
                        ) {
                            log::error!("Impossible to save profile: {}", e.to_string());
                        }
                    }
                }
            } else if let Ok(metadata) = Metadata::from_json(&event.content) {
                if let Err(e) = store.set_profile(
                    event.pubkey,
                    Profile {
                        metadata,
                        timestamp: event.created_at,
                    },
                ) {
                    log::error!("Impossible to save profile: {}", e.to_string());
                }
            }
        }
        Kind::Base(KindBase::ContactList) => {
            let mut contact_list: Vec<Contact> = Vec::new();
            for tag in event.tags.clone().into_iter() {
                let tag: Vec<String> = tag.as_vec();
                if let Some(pk) = tag.get(1) {
                    if let Ok(pk) = XOnlyPublicKey::from_str(pk) {
                        authors.push(pk);

                        let relay_url = tag.get(2).cloned();
                        let alias = tag.get(3).cloned();
                        contact_list.push(Contact::new(
                            pk,
                            relay_url.unwrap_or_default(),
                            alias.unwrap_or_default(),
                        ));
                    }
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

    if let Err(e) = store.set_authors(authors) {
        log::error!("Impossible to save authors: {}", e.to_string());
    }
}
