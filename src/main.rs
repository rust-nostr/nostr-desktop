// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::{clipboard, executor, Application, Command, Element, Settings, Subscription, Theme};

mod component;
mod context;
mod layout;
mod message;
mod nostr;
mod theme;

use self::context::{Context, Menu, Setting, Stage};
use self::layout::{
    ChatState, ContactsState, ExploreState, HomeState, LoginState, NotificationsState,
    ProfileState, RelaysState, SettingState, State,
};
use self::message::Message;
use self::nostr::sync::NostrSync;

pub fn main() -> iced::Result {
    env_logger::init();
    let mut settings = Settings::default();
    settings.window.min_size = Some((600, 600));
    App::run(settings)
}

struct App {
    state: Box<dyn State>,
    context: Context,
}

pub fn new_state(context: &Context) -> Box<dyn State> {
    match &context.stage {
        Stage::Login => LoginState::new().into(),
        Stage::Register => todo!(),
        Stage::Menu(menu) => match menu {
            Menu::Home => HomeState::new().into(),
            Menu::Explore => ExploreState::new().into(),
            Menu::Chats => ChatState::new().into(),
            Menu::Contacts => ContactsState::new().into(),
            Menu::Notifications => NotificationsState::new().into(),
            Menu::Profile => ProfileState::new().into(),
            Menu::Setting(s) => match s {
                Setting::Main => SettingState::new().into(),
                Setting::Relays => RelaysState::new().into(),
            },
        },
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        // read local db
        // if key exists, load main app
        // else load login/register view
        let context = Context::new(Stage::default(), None);
        let app = Self {
            state: new_state(&context),
            context,
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        self.state.title()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        if let Some(client) = self.context.client.clone() {
            let sync = NostrSync::subscription(client).map(Message::Sync);
            Subscription::batch(vec![sync, self.state.subscription()])
        } else {
            Subscription::batch(vec![self.state.subscription()])
        }
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::SetStage(stage) => {
                self.context.set_stage(stage);
                self.state = new_state(&self.context);
                self.state.update(&mut self.context, message)
            }
            Message::Clipboard(text) => clipboard::write(text),
            _ => self.state.update(&mut self.context, message),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        self.state.view(&self.context)
    }
}
