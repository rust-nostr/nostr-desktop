// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::{clipboard, executor, Application, Command, Element, Settings, Theme};

mod component;
mod context;
mod layout;
mod theme;

use self::context::{Context, Menu, Setting, Stage};
use self::layout::{
    HomeMessage, HomeState, LoginMessage, LoginState, RelaysMessage, RelaysState, State,
};

pub fn main() -> iced::Result {
    env_logger::init();
    App::run(Settings::default())
}

struct App {
    state: Box<dyn State>,
    context: Context,
}

pub fn new_state(context: &Context) -> Box<dyn State> {
    match &context.stage {
        Stage::Login => LoginState::new().into(),
        Stage::Menu(menu) => match menu {
            Menu::Home => HomeState::new().into(),
            Menu::Setting(s) => match s {
                Setting::Main => todo!(),
                Setting::Relays => RelaysState::new().into(),
            },
        },
        _ => todo!(),
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetStage(Stage),
    Clipboard(String),
    Login(LoginMessage),
    Home(HomeMessage),
    Relays(RelaysMessage),
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
