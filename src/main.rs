use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};
use iced::widget::{button, Button,Column, Text, ProgressBar, text};

fn main() {
    GUIApp::run(Settings::default()).expect("Could not run application");
}
#[derive(Debug, Clone)]
pub enum Message{
    ButtonPressed,
}

struct GUIApp{
    button: iced::widget::button::State,
}

impl Application for GUIApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;
    fn new(_flags: ()) -> (GUIApp, Command<Self::Message>) {
        (GUIApp{ button: iced::widget::button::State::new()}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Install")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::ButtonPressed => {
                println!("Testing action");
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let content = Column::new()
            .spacing(20) // Spacing between elements
            .push(Button::new(Text::new("Install")).on_press(Message::ButtonPressed))
            .push(Button::new(Text::new("Cancel"))) 
            .push(Text::new("App directory"))
            .push(ProgressBar::new(0.0..=100.0, 50.0));

        // Convert the layout into an Element to be displayed
        content.into()
    }

    
}