// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::widget::{image, Column, Container, Row, Text};
use iced::{Alignment, Length};
use nostr_sdk::nostr::secp256k1::XOnlyPublicKey;
use nostr_sdk::nostr::Metadata;
use once_cell::sync::Lazy;

static UNKNOWN_IMG_PROFILE: Lazy<image::Handle> = Lazy::new(|| {
    image::Handle::from_memory(
        include_bytes!("../../../../static/imgs/unknown-img-profile.png").to_vec(),
    )
});

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
        let image = self
            .image
            .clone()
            .unwrap_or_else(|| UNKNOWN_IMG_PROFILE.to_owned());
        let image = Column::new().push(
            image::viewer(image)
                .height(Length::Units(40))
                .width(Length::Units(40)),
        );

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
