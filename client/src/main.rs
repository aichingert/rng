use iced::*;
use iced::widget::{
    container, column,
    canvas::{self, Event, Canvas, Cache, Frame, Geometry, Path, Stroke}
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
    pub cache: Cache,
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

const CELL_SIZE: u16 = 50;

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
        let (w, h) = (bounds.width, bounds.height);

        let grid = self.cache.draw(renderer, bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, Color::from_rgb8(0x40, 0x44, 0x4B));

            frame.with_save(|frame| {
                frame.scale(CELL_SIZE);

                let x = (w / 3.) / (CELL_SIZE as f32);
                let y = (h / 2.) / (CELL_SIZE as f32);

                for i in 1..=2 {
                    let hor_src = Point::new((i as f32) * x, y - (y / 1.5));
                    let hor_dst = Point::new((i as f32) * x, y + (y / 1.5));


                    let ver_src = Point::new(y - (y / 1.5), (i as f32) * x);
                    let ver_dst = Point::new(2.5 * y, (i as f32) * x);

                    frame.stroke(&Path::line(hor_src, hor_dst), Stroke::default().with_color(Color::new(1., 1., 1., 1.)));
                    frame.stroke(&Path::line(ver_src, ver_dst), Stroke::default().with_color(Color::new(1., 1., 1., 1.)));
                }

            });
        });

        vec![grid]
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
                grid: Grid { fields: vec![], cache: Cache::new() }
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
