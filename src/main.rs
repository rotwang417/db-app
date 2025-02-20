use iced::{executor, Settings};
use iced::{Application, Command, Element, Length};
use iced::widget::{Button, button, Column, Container, Text};
use rand::Rng;
use rusqlite::{params, Connection, Result};

#[derive(Default)]
struct CitiesStates {
    message: String,
    button_state: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    FetchWord,
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
            // .push(Button::new(&mut self.button_state).on_press(Message::FetchWord))
            // .push(button(&mut self.button_state).on_press(Message::FetchWord))
            .push(Button::new(Text::new("Fetch Random Word")).on_press(Message::FetchWord))

            // .push(button(&mut self.button_state, Text::new("Fetch random word")).on_press(Message::FetchWord))
            .into()
    }
}

fn fetch_random_word() -> Result<String> {
    println!("Current directory: {:?}", std::env::current_dir());
    let connection = match Connection::open("resources/database.db")
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to open database: {}", e);
            return Err(e);
        }
    };
    println!("Database connection established.");

    let mut stmt = match connection.prepare("SELECT text FROM messages") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to build query: {}", e);
            return Err(e);
        }
    };;
    let words: Vec<String> = match stmt.query_map([], |row| row.get(0)) {
        Ok(rows) => rows.filter_map(Result::ok).collect(),
        Err(e) => {
            eprintln!("Failed to query data: {}", e);
            return Err(e);
        }
    };

    if words.is_empty() {
        return Ok(String::from("No words found"));
    }
    let random_index = rand::thread_rng().gen_range(0..words.len());
    Ok(words[random_index].clone())
}

fn main() -> iced::Result {
    CitiesStates::run(Settings::default())
}