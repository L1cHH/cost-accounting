use iced::{Background, Border, Color, Theme};
use iced::border::Radius;

use iced::widget::{button, container};
use iced::widget::container::Appearance;

// CONTAINERS
pub struct NotesContainer;

impl container::StyleSheet for NotesContainer {
    type Style = Theme;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgba8(189, 147, 249, 0.3))),
            border: Border {
                color: Color::BLACK,
                width: 1.0,
                radius: Radius::from(20)
            },
            ..Default::default()
        }
    }
}

pub struct CategoryContainer;

impl container::StyleSheet for CategoryContainer {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgba8(98, 114, 164, 0.3))),
            border: Border {
                color: Color::BLACK,
                width: 1.0,
                radius: Radius::from(20)
            },
            ..Default::default()
        }
    }
}

pub struct TestContainer;

impl container::StyleSheet for TestContainer {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border: Border {
                color: Color::BLACK,
                width: 1.0,
                radius: Radius::from(20)
            },
            ..Default::default()
        }
    }
}

//BUTTONS

