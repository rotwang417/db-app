use iced::{executor, Settings};
use iced::{Application, Command, Element, Length};
use iced::widget::{Button, button, Column, Container, Text};

use crate::messages::Message;
use crate::database::fetch_random_word;
use crate::views::{MenuOption, View};

#[derive(Default)]
pub struct CitiesStates {
    message: String,
    button_state: button::State,
}

impl Application for CitiesStates {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { message: String::from("Fetch text"), button_state: button::State::new() }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Cities, States")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::FetchWord => {
                if let Ok(random_word) = fetch_random_word() {
                    self.message = random_word;
                } else {
                    self.message = String::from("Error fetching word");
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new(&self.message))
            .push(Button::new(Text::new("Fetch Random Word")).on_press(Message::FetchWord))
            .into()
    }
}