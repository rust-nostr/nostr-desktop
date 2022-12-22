// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::{Command, Element, Subscription};

mod login;
mod menu;

pub use self::login::{LoginMessage, LoginState};
pub use self::menu::*;
use crate::context::Context;
use crate::message::Message;

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
