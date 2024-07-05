use iced::{Color, Element, Renderer, Theme, widget};
use iced::theme::Button;
use iced::widget::{Component, component, text};
use crate::custom_widgets::exit_button_widget::ExitButtonStyle;
use super::super::Message;
use crate::pages::notes_page::NotesCategory;




pub struct Hyperlink<Message>{
    placeholder: String,
    category_to: NotesCategory,
    on_change: Box<dyn Fn(NotesCategory) -> Message>
}

impl Hyperlink<Message> {
    pub fn new(placeholder: String, category_to: NotesCategory, on_change: impl Fn(NotesCategory) -> Message + 'static) -> Self {
        Hyperlink {
            placeholder,
            category_to,
            on_change: Box::new(on_change)
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum HyperlinkEvent {
    Clicked,
    MouseEnter,
    MouseExit
}
#[derive(Default)]
pub struct HyperlinkState {
    hovered: bool
}

impl <Message> Component<Message> for Hyperlink<Message> {
    type Event = HyperlinkEvent;
    type State = HyperlinkState;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            HyperlinkEvent::Clicked => Some((self.on_change)(self.category_to)),
            HyperlinkEvent::MouseEnter => {
                state.hovered = true;
                None
            },
            HyperlinkEvent::MouseExit => {
                state.hovered = false;
                None
            }
        }
    }

    fn view(&self, state: &Self::State) -> iced::Element<'_, Self::Event, Theme, Renderer> {
        widget::button(
            widget::mouse_area(text(&self.placeholder).size(20).style(iced::theme::Text::Color(
                if state.hovered {
                    Color::from_rgba8(189, 147, 249, 0.8)
                } else {
                    Color::from_rgba8(189, 147, 249, 0.3)
                }
            )))
                .on_press(HyperlinkEvent::Clicked)
                .on_enter(HyperlinkEvent::MouseEnter)
                .on_exit(HyperlinkEvent::MouseExit)
        )
            .padding(5)
            .style(iced::theme::Button::Custom(Box::new(ExitButtonStyle)))
            .on_press(HyperlinkEvent::Clicked)
            .into()
    }
}

impl <'a, Message> From <Hyperlink<Message>> for Element<'a, Message, Theme, Renderer> where Message:'a {
    fn from(value: Hyperlink<Message>) -> Self {
        component(value)
    }
}
