use iced::{Background, Border, Color, Theme, widget};
use iced::border::Radius;

use iced::widget::{button, container, text_input};
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

//TEXT INPUTS

pub struct ErrorTextInputStyle;

impl text_input::StyleSheet for ErrorTextInputStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: Color::from_rgba8(206, 44, 44, 0.8),
                width: 2.0,
                radius: Radius::from(4),
            },
            background: Background::Color(Color::from_rgba8(58, 53, 59, 0.8)),
            icon_color: Color::from_rgba8(241, 214, 222, 0.8)
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: Color::from_rgba8(243, 226, 236, 0.8),
                width: 2.0,
                radius: Radius::from(4),
            },
            background: Background::Color(Color::from_rgba8(58, 53, 59, 0.8)),
            icon_color: Color::from_rgba8(241, 214, 222, 0.8)
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: Color::from_rgba8(167, 14, 60, 0.8),
                ..Border::default()
            },
            background: Background::Color(Color::from_rgba8(58, 53, 59, 0.8)),
            icon_color: Color::from_rgba8(241, 214, 222, 0.8)
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: Color::from_rgba8(206, 44, 44, 0.8),
                width: 2.0,
                radius: Radius::from(4),
            },
            background: Background::Color(Color::from_rgba8(58, 53, 59, 0.8)),
            icon_color: Color::from_rgba8(241, 214, 222, 0.8)
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        Color::from_rgba8(241, 214, 222, 0.8)
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        Color::from_rgba8(241, 214, 222, 0.8)
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        Color::BLACK
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        Color::WHITE
    }
}

pub struct CorrectTextInputStyle;

impl text_input::StyleSheet for CorrectTextInputStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: Color::from_rgba8(24, 212, 80, 0.8),
                width: 2.0,
                radius: Radius::from(4),
            },
            background: Background::Color(Color::from_rgba8(58, 53, 59, 0.8)),
            icon_color: Color::from_rgba8(241, 214, 222, 0.8)
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: Color::from_rgba8(243, 226, 236, 0.8),
                width: 2.0,
                radius: Radius::from(4),
            },
            background: Background::Color(Color::from_rgba8(58, 53, 59, 0.8)),
            icon_color: Color::from_rgba8(241, 214, 222, 0.8)
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: Color::from_rgba8(167, 14, 60, 0.8),
                ..Border::default()
            },
            background: Background::Color(Color::from_rgba8(58, 53, 59, 0.8)),
            icon_color: Color::from_rgba8(241, 214, 222, 0.8)
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: Color::from_rgba8(24, 212, 80, 0.8),
                width: 2.0,
                radius: Radius::from(4),
            },
            background: Background::Color(Color::from_rgba8(58, 53, 59, 0.8)),
            icon_color: Color::from_rgba8(241, 214, 222, 0.8)
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        Color::from_rgba8(241, 214, 222, 0.8)
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        Color::from_rgba8(241, 214, 222, 0.8)
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        Color::BLACK
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        Color::WHITE
    }

}

