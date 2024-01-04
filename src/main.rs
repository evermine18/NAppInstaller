mod counter;
use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};
use iced::widget::{Button, text};
use counter::Counter;
fn main() {
    GUIApp::run(Settings::default()).expect("Could not run application");
}

struct GUIApp;

impl Application for GUIApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;
    fn new(_flags: ()) -> (GUIApp, Command<Self::Message>) {
        (GUIApp, Command::none())
    }

    fn title(&self) -> String {
        String::from("Install")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        button("Install").into()
    }

    
}
fn button<'a, Message: Clone>(label: &str) -> Button<'a, Message> {
    iced::widget::button(
        text(label),
    )
    .padding(12)
    .width(100)
}