mod installer;
mod run;
mod args;
use iced::{executor, Length, Alignment};
use iced::{Application, Command, Element, Settings, Theme};
use iced::widget::{button, Button,Column, Text, ProgressBar, text, Row, Space};
use installer::install;
use std::{env, vec::Vec};
use run::run_app;

fn main() {
    //GUIApp::run(Settings::default()).expect("Could not run application");
    let app_name = args::get_appimage_path();
    if args::is_install(){
        install(&app_name);
    }else{
        run_app(&app_name);
    }
    //test_lib();
}
#[derive(Debug, Clone)]
pub enum Message{
    ButtonPressed,
}

struct GUIApp{

}

impl Application for GUIApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;
    fn new(_flags: ()) -> (GUIApp, Command<Self::Message>) {
        (GUIApp{ }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Install")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::ButtonPressed => {
                println!("Installing...");
                let args: Vec<String> = env::args().collect();
                if args.len() > 1 && args[1] != "" {
                    install(&args[1]);
                }
                else{
                    println!("[ERROR] No appimage provided");
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let button_container = Row::new()
            .spacing(20)
            .push(Button::new(Text::new("Install")).on_press(Message::ButtonPressed))
            .push(Button::new(Text::new("Cancel")))
            .align_items(Alignment::Center); // Center the contents horizontally
    
        let content = Column::new()
            .push(Text::new("app_name").size(50)) // App name
            .push(Space::with_height(Length::FillPortion(1))) // Spacer to center vertically
            .push(button_container)
            .push(Space::with_height(Length::FillPortion(1))) // Spacer to center vertically
            .width(Length::Fill)   // Filling the width of the window
            .height(Length::Fill)  // Same for the height
            .align_items(Alignment::Center); // Center the contents horizontally
    
        // Convert the content into a container with a padding
        content.into()
    }

    
}