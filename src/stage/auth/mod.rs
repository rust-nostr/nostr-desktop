// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::{Command, Element, Subscription};

mod context;
pub mod screen;

pub use self::context::{Context, Stage};
use self::screen::LoginState;
use crate::{message::Message, NostrDesktop};

use super::App;

pub struct Auth {
    state: Box<dyn State>,
    context: Context,
}

pub fn new_state(context: &Context) -> Box<dyn State> {
    match &context.stage {
        Stage::Login => LoginState::new().into(),
        Stage::Register => todo!(),
    }
}

pub trait State {
    fn title(&self) -> String;
    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message>;
    fn view(&self, ctx: &Context) -> Element<Message>;
    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
    fn load(&self, _ctx: &Context) -> Command<Message> {
        Command::none()
    }
}

impl Auth {
    pub fn new() -> (Self, Command<Message>) {
        // read local db
        // if key exists, load main app
        // else load login/register view
        let context = Context::new(Stage::default());
        let app = Self {
            state: new_state(&context),
            context,
        };
        (app, Command::none())
    }

    pub fn title(&self) -> String {
        self.state.title()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![self.state.subscription()])
    }

    pub fn update(&mut self, message: Message) -> (Command<Message>, Option<NostrDesktop>) {
        match message {
            Message::SetAuthStage(stage) => {
                self.context.set_stage(stage);
                self.state = new_state(&self.context);
                (self.state.update(&mut self.context, message), None)
            }
            Message::LoginResult(client, store) => {
                let app = App::new(client, store);
                (app.1, Some(NostrDesktop::Dashboard(app.0)))
            }
            _ => (self.state.update(&mut self.context, message), None),
        }
    }

    pub fn view(&self) -> Element<Message> {
        self.state.view(&self.context)
    }
}
