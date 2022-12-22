// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use async_stream::stream;
use iced::Subscription;
use iced_futures::BoxStream;
use nostr_sdk::blocking::Client;
use nostr_sdk::nostr::Event;
use nostr_sdk::RelayPoolNotifications;
use tokio::sync::mpsc;

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
        let join = tokio::task::spawn(async move {
            loop {
                let mut notifications = client.notifications();
                while let Ok(notification) = notifications.recv().await {
                    if let RelayPoolNotifications::ReceivedEvent(event) = notification {
                        sender.send(event).ok();
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
        Subscription::from_recipe(NostrSync { client, join: None })
    }
}
