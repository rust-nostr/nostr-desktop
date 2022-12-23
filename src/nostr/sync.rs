// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use async_stream::stream;
use iced::Subscription;
use iced_futures::BoxStream;
use nostr_sdk::blocking::Client;
use nostr_sdk::nostr::{Event, Kind, KindBase, SubscriptionFilter};
use nostr_sdk::RelayPoolNotifications;
use tokio::sync::mpsc;

use crate::nostr::db::Store;

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
        let store = self.store;

        let subscription = SubscriptionFilter::new()
            .pubkey(client.keys().public_key())
            .kind(Kind::Base(KindBase::EncryptedDirectMessage));

        if let Err(e) = client.subscribe(vec![subscription]) {
            log::error!("Impossible to subscribe to events: {}", e.to_string());
        }

        let join = tokio::task::spawn(async move {
            loop {
                let mut notifications = client.notifications();
                while let Ok(notification) = notifications.recv().await {
                    if let RelayPoolNotifications::ReceivedEvent(event) = notification {
                        if let Err(e) = store.save_events(vec![event.clone()]) {
                            log::error!("Impossible to save event: {}", e.to_string());
                        }
                        sender.send(event).ok();
                    }
                }
                store.flush();
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
