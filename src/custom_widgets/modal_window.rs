use iced::advanced::layout::{self, Layout, Node};
use iced::advanced::{overlay, renderer};
use iced::advanced::renderer::Style;
use iced::Renderer;
use iced::Theme;
use iced::advanced::layout::Limits;
use iced::advanced::widget::{self, Widget};
use iced::advanced::{self, Clipboard, Shell};
use iced::advanced::widget::{Tree};
use iced::alignment::Alignment;
use iced::event;
use iced::mouse::{self, Cursor, Interaction};
use iced::theme;
use iced::{Color, Element, Event, Length, Point, Rectangle, Size, Vector};
use iced::event::Status;


pub struct Modal<'a, Message, Theme, Renderer> {
    outer_content: Element<'a, Message, Theme, Renderer>,
    inner_content: Element<'a, Message, Theme, Renderer>,
    on_blur: Option<Message>
}

impl <'a, Message, Theme, Renderer> Modal<'a, Message, Theme, Renderer> {

    pub fn new(
        outer_content: impl Into<Element<'a, Message, Theme, Renderer>>,
        inner_content: impl Into<Element<'a, Message, Theme, Renderer>>
    ) -> Self {
        Modal {
            outer_content: outer_content.into(),
            inner_content: inner_content.into(),
            on_blur: None
        }
    }

    pub fn on_blur(self, on_blur: Message) -> Self {
        Self {
            on_blur: Some(on_blur),
            ..self
        }
    }

}

impl <'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Modal<'a, Message, Theme, Renderer> where Renderer: advanced::Renderer, Message: Clone {
    fn children(&self) -> Vec<Tree> {
        vec![
            Tree::new(&self.outer_content),
            Tree::new(&self.inner_content)
        ]
    }

    fn diff(&self, _tree: &mut Tree) {
        _tree.diff_children(&[&self.outer_content, &self.inner_content]);
    }

    fn size(&self) -> Size<Length> {
        self.outer_content.as_widget().size()
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> layout::Node {
        self.outer_content.as_widget().layout(&mut tree.children[0], renderer, limits)
    }
    fn draw(&self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, style: &Style, layout: Layout<'_>, cursor: Cursor, viewport: &Rectangle) {
        self.outer_content.as_widget().draw(&tree.children[0], renderer, theme, style, layout, cursor, viewport)
    }

    fn overlay<'b>(
        &'b mut self,
        _state: &'b mut Tree,
        _layout: Layout<'_>,
        _renderer: &Renderer,
        _translation: Vector
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        Some(overlay::Element::new(Box::new(Overlay {
            content: &mut self.inner_content,
            tree: &mut _state.children[1],
            size: _layout.bounds().size(),
            on_blur: self.on_blur.clone()
        })))
    }
    
}

struct Overlay<'a, 'b, Message, Theme, Renderer> {
    content: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut Tree,
    size: Size,
    on_blur: Option<Message>
}

impl <'a, 'b, Message, Theme, Renderer>
    overlay::Overlay<Message, Theme, Renderer>
    for Overlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: advanced::Renderer,
    Message: Clone
{
    fn layout(
        &mut self,
        renderer: &Renderer,
        bounds: Size
    ) -> Node {
        let limits = Limits::new(Size::ZERO, self.size)
            .width(Length::Fill)
            .height(Length::Fill);

        let child = self.content
            .as_widget_mut()
            .layout(self.tree, renderer, &limits)
            .align(Alignment::Center, Alignment::Center, limits.max());

        layout::Node::with_children(self.size, vec![child])
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor
    ) {

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                ..renderer::Quad::default()
            },
            Color {
                a: 0.80,
                ..Color::BLACK
            }
        );


        self.content.as_widget().draw(self.tree, renderer, theme, style, layout.children().next().unwrap(), cursor, &layout.bounds());
    }

    fn on_event(&mut self, _event: Event, _layout: Layout<'_>, _cursor: Cursor, _renderer: &Renderer, _clipboard: &mut dyn Clipboard, _shell: &mut Shell<'_, Message>) -> Status {

        let content_bounds = _layout.children().next().unwrap().bounds();

        if let Some(message) = self.on_blur.as_ref() {
            if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = _event {
                if !_cursor.is_over(content_bounds) {
                    _shell.publish(message.clone());
                    return Status::Captured;
                }
            }
        };

        self.content.as_widget_mut().on_event(
            self.tree,
            _event,
            _layout.children().next().unwrap(),
            _cursor,
            _renderer,
            _clipboard,
            _shell,
            &_layout.bounds()
        )
    }

    fn mouse_interaction(
        &self,
        _layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer
    ) -> Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            _layout.children().next().unwrap(),
            _cursor,
            _viewport,
            _renderer
        )
    }
}

impl <'a, Message, Theme, Renderer> From<Modal<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: 'a,
    Renderer: 'a + advanced::Renderer,
    Message: 'a + Clone
{
    fn from(value: Modal<'a, Message, Theme, Renderer>) -> Self {
        Element::new(value)
    }
}

