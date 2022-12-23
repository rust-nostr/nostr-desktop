// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{Column, Container, Row, Text};
use iced::Alignment;
use nostr_sdk::nostr::secp256k1::XOnlyPublicKey;
use nostr_sdk::nostr::Metadata;

pub struct Contact {
    public_key: XOnlyPublicKey,
    metadata: Metadata,
}

impl Contact {
    pub fn new(public_key: XOnlyPublicKey, metadata: Metadata) -> Self {
        Self {
            public_key,
            metadata,
        }
    }

    pub fn view<'a, T: 'a>(self) -> Container<'a, T> {
        let mut image = Column::new();

        if let Some(picture) = self.metadata.picture {
            image = image.push(Text::new(picture.to_string()));
        } else {
            image = image.push(Text::new("No image"));
        }

        let mut info = Column::new();

        if let Some(display_name) = self.metadata.display_name {
            info = info.push(Row::new().push(Text::new(display_name)));
        } else {
            let pk = self.public_key.to_string();
            info = info.push(Row::new().push(Text::new(format!(
                "{}:{}",
                &pk[0..8],
                &pk[pk.len() - 8..]
            ))));
        }

        if let Some(name) = self.metadata.name {
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
