// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use nostr_sdk::nostr::SubscriptionFilter;

pub struct Filters {
    pub contact_list: SubscriptionFilter,
    pub encrypted_dm: SubscriptionFilter,
    pub following_authors: SubscriptionFilter,
}

impl Filters {
    pub fn to_vec(&self) -> Vec<SubscriptionFilter> {
        vec![
            self.contact_list.clone(),
            self.encrypted_dm.clone(),
            self.following_authors.clone(),
        ]
    }
}
