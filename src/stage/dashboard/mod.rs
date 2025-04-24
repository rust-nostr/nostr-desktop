// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::{Command, Element, Subscription};
use nostr_sdk::Client;

pub mod component;
mod context;
pub mod screen;

pub use self::context::{Context, Setting, Stage};
use self::screen::{
    ChatState, ContactsState, ExploreState, HomeMessage, HomeState, NotificationsState,
    ProfileState, RelaysState, SettingState,
};
use crate::message::Message;
use crate::sync::NostrSync;

pub struct App {
    pub state: Box<dyn State>,
    pub context: Context,
}

pub fn new_state(context: &Context) -> Box<dyn State> {
    match &context.stage {
        Stage::Home => HomeState::new().into(),
        Stage::Explore => ExploreState::new().into(),
        Stage::Chats => ChatState::new().into(),
        Stage::Contacts => ContactsState::new().into(),
        Stage::Notifications => NotificationsState::new().into(),
        Stage::Profile => ProfileState::new().into(),
        Stage::Setting(s) => match s {
            Setting::Main => SettingState::new().into(),
            Setting::Relays => RelaysState::new().into(),
        },
    }
}

pub trait State {
    fn title(&self) -> String;
    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message>;
    fn view(&self, ctx: &Context) -> Element<Message>;
    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
    fn load(&mut self, _ctx: &Context) -> Command<Message> {
        Command::none()
    }
}

impl App {
    pub fn new(client: Client) -> (Self, Command<Message>) {
        let context = Context::new(Stage::default(), client.clone());
        let app = Self {
            state: new_state(&context),
            context,
        };
        (
            app,
            Command::perform(
                async move {
                    if let Err(e) = client.restore_relays().await {
                        log::error!("Impossible to load relays: {}", e);
                    }
                    client.connect().await;
                },
                |_| Message::Tick,
            ),
        )
    }

    pub fn title(&self) -> String {
        self.state.title()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let sync = NostrSync::subscription(self.context.client.clone()).map(Message::Sync);
        Subscription::batch(vec![sync, self.state.subscription()])
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SetDashboardStage(stage) => {
                self.context.set_stage(stage);
                self.state = new_state(&self.context);
                self.state.update(&mut self.context, message)
            }
            Message::Sync(event) => match self.context.stage {
                Stage::Home => self
                    .state
                    .update(&mut self.context, HomeMessage::PushTextNote(event).into()),
                _ => Command::none(),
            },
            _ => self.state.update(&mut self.context, message),
        }
    }

    pub fn view(&self) -> Element<Message> {
        self.state.view(&self.context)
    }
}
