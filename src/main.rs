use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

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
        "Test".into()
    }
}