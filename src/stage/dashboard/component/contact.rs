// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{image, Column, Container, Row, Text};
use iced::{Alignment, Length};
use nostr_sdk::nostr::secp256k1::XOnlyPublicKey;
use nostr_sdk::nostr::Metadata;

#[derive(Debug)]
pub struct Contact {
    pub public_key: XOnlyPublicKey,
    pub metadata: Metadata,
    pub image: Option<image::Handle>,
}

impl Contact {
    pub fn new(public_key: XOnlyPublicKey, metadata: Metadata) -> Self {
        Self {
            public_key,
            metadata,
            image: None,
        }
    }

    pub fn view<'a, T: 'a>(&'a self) -> Container<'a, T> {
        let image = if let Some(image) = self.image.clone() {
            Column::new().push(
                image::viewer(image)
                    .height(Length::Units(40))
                    .width(Length::Units(40)),
            )
        } else {
            Column::new().push(Text::new("No image"))
        };

        let mut info = Column::new();

        if let Some(display_name) = self.metadata.display_name.clone() {
            info = info.push(Row::new().push(Text::new(display_name)));
        } else {
            let pk = self.public_key.to_string();
            info = info.push(Row::new().push(Text::new(format!(
                "{}:{}",
                &pk[0..8],
                &pk[pk.len() - 8..]
            ))));
        }

        if let Some(name) = self.metadata.name.clone() {
            info = info.push(Row::new().push(Text::new(format!("@{}", name)).size(16)));
        } else {
            info = info.push(Row::new());
        }

        let row = Row::new()
            .push(image)
            .push(info)
            .align_items(Alignment::Center)
            .spacing(20);
        Container::new(row)
    }
}
