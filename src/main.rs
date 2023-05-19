use iced::widget::{container, text};
use iced::executor;
use iced::{Application, Element, Settings, Theme, Command};

#[derive(Default)]
struct IcedTwentyOne;

#[derive(Debug, Clone, Copy)]
enum Message {
}

impl Application for IcedTwentyOne {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (IcedTwentyOne, Command<Self::Message>) {
        (IcedTwentyOne::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Iced Twenty-One")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let hello = text("Hello, Iced! (with application trait)");
        container(hello).into()
    }
}

#[feature(windows_subsystem)]
#[windows_subsystem = "windows"]
pub fn main() -> iced::Result {
    IcedTwentyOne::run(Settings::default())
}