// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::{mouse, Border, Color, Element, Length, Rectangle, Size};

use crate::theme::color::BLACK;

pub struct Circle {
    radius: f32,
    color: Color,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            color: BLACK,
        }
    }

    pub fn color(self, color: Color) -> Self {
        Self { color, ..self }
    }
}

/* pub fn circle(radius: f32) -> Circle {
    Circle::new(radius)
} */

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Circle
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border {
                    width: 0.0,
                    color: Color::TRANSPARENT,
                    radius: self.radius.into(),
                },
                ..renderer::Quad::default()
            },
            self.color,
        );
    }
}
#[allow(clippy::needless_lifetimes)]
impl<'a, Message, Theme, Renderer> From<Circle> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(circle: Circle) -> Self {
        Self::new(circle)
    }
}
