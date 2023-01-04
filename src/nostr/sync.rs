// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::time::Duration;

use async_stream::stream;
use iced::Subscription;
use iced_futures::BoxStream;
use nostr_sdk::nostr::{Event, Kind, KindBase, SubscriptionFilter};
use nostr_sdk::Client;
use nostr_sdk::RelayPoolNotifications;
use tokio::sync::mpsc;

use super::filters::Filters;
use crate::RUNTIME;

pub struct NostrSync {
    client: Client,
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

            if let Ok(store) = client.store() {
                if let Ok(mut authors) = store.get_authors() {
                    if !authors.contains(&my_public_key) {
                        authors.push(my_public_key);
                    }
                    filters.following_authors =
                        SubscriptionFilter::new().authors(authors).kinds(vec![
                            Kind::Base(KindBase::Metadata),
                            Kind::Base(KindBase::TextNote),
                            Kind::Base(KindBase::Boost),
                            Kind::Base(KindBase::Reaction),
                        ]);
                }
            }

            if let Err(e) = RUNTIME.block_on(async { client.subscribe(filters.to_vec()).await }) {
                log::error!("Impossible to subscribe to events: {}", e.to_string());
            }

            loop {
                std::thread::sleep(Duration::from_secs(30));

                if let Ok(store) = client.store() {
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
            }
        });

        let client = self.client.clone();
        let join = tokio::task::spawn(async move {
            loop {
                let mut notifications = client.notifications();
                while let Ok(notification) = notifications.recv().await {
                    match notification {
                        RelayPoolNotifications::ReceivedEvent(event) => {
                            // Send desktop notification
                            if let Ok(store) = client.store() {
                                if let Err(e) = store.handle_event(&event) {
                                    log::error!("Impossible to handle event: {}", e.to_string());
                                }
                            }

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
    pub fn subscription(client: Client) -> Subscription<Event> {
        Subscription::from_recipe(Self { client, join: None })
    }
}
