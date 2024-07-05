use std::path::PathBuf;
use iced::{Background, Border, Color, Element, Renderer, Theme};
use iced::border::Radius;
use iced::widget::{button, Component, component, Svg};
use iced::widget;
use super::super::Message;

pub struct ExitButton <Message> {
    active_view: PathBuf,
    hovered_view: PathBuf,
    clicked_view: PathBuf,
    on_click: Box<dyn Fn(()) -> Message>
}
impl ExitButton<Message> {
    pub fn new(active_svg: PathBuf, hovered_svg: PathBuf, clicked_svg: PathBuf, on_click: impl Fn(()) -> Message + 'static) -> Self {
        ExitButton {
            active_view: active_svg,
            hovered_view: hovered_svg,
            clicked_view: clicked_svg,
            on_click: Box::new(on_click)
        }
    }
}



pub enum ExitButtonState {
    Hovered(bool),
    Clicked
}
impl Default for ExitButtonState {
    fn default() -> Self {
        ExitButtonState::Hovered(false)
    }
}

#[derive(Clone)]
pub enum ExitButtonEvent {
    Clicked,
    MouseEnter,
    MouseExit
}

impl <Message> Component<Message> for ExitButton<Message> {
    type Event = ExitButtonEvent;
    type State = ExitButtonState;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            ExitButtonEvent::MouseEnter => {
                *state = ExitButtonState::Hovered(true);
                None
            },

            ExitButtonEvent::MouseExit => {
                *state = ExitButtonState::Hovered(false);
                None
            },

            ExitButtonEvent::Clicked => {
                *state = ExitButtonState::Clicked;
                Some((self.on_click)(()))
            }
        }
    }

    fn view(&self, state: &Self::State) -> iced::Element<'_, Self::Event, Theme, Renderer> {
        widget::button(
            widget::mouse_area(
                match state {
                    ExitButtonState::Hovered(true) => {
                        Svg::from_path(&self.hovered_view).height(50)
                    }
                    ExitButtonState::Hovered(false) => {
                        Svg::from_path(&self.active_view).height(50)
                    }
                    ExitButtonState::Clicked => {
                        Svg::from_path(&self.clicked_view).height(50)
                    }
                }

            )
                .on_press(ExitButtonEvent::Clicked)
                .on_enter(ExitButtonEvent::MouseEnter)
                .on_exit(ExitButtonEvent::MouseExit)
        )
            .style(iced::theme::Button::Custom(Box::new(ExitButtonStyle)))
            .on_press(ExitButtonEvent::Clicked)
            .into()
    }
}

impl <'a, Message> From<ExitButton<Message>> for Element<'a, Message> where Message: 'a {
    fn from(value: ExitButton<Message>) -> Self {
        component(value)
    }
}

pub struct ExitButtonStyle;

impl button::StyleSheet for ExitButtonStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border: Border {
                color: Color::BLACK,
                width: 0.0,
                radius: Radius::from(20)
            },
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: None,
            border: Border {
                color: Color::BLACK,
                width: 0.0,
                radius: Radius::from(20)
            },
            ..Default::default()
        }
    }
}