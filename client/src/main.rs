use iced::*;
use iced::widget::{
    container, column,
    canvas::{self, Event, Canvas, Frame, Geometry}
};

pub fn main() -> iced::Result{
    Toc::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct Toc {
    grid: Grid,
}

#[derive(Default)]
enum Field {
    Rec(Box<Grid>),
    Rel(Vec<bool>),
    #[default]
    Def,
}

#[derive(Default)]
struct Grid {
    pub fields: Vec<Field>,
}

impl Grid {
    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[derive(Default)]
enum Interaction { 
    #[default]
    N 
}

impl canvas::Program<Message> for Grid {
    type State = Interaction;

    fn update(
        &self, 
        interaction: &mut Interaction,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<Message>) {
        (event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _interaction: &Interaction,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        vec![]
    }

    fn mouse_interaction(
        &self,
        interaction: &Interaction,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::default()
    }
}

#[derive(Debug, Clone)]
enum Message {}

impl Application for Toc {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                grid: Grid { fields: vec![] }
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Toc")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&self) -> Element<Message> {

        let content = column![self.grid.view()]
            .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
