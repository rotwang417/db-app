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
            .push(Button::new(Text::new("Fetch Random Word")).on_press(Message::FetchWord))
            .into()
    }
}

fn fetch_random_word() -> Result<String> {
    println!("Current directory: {:?}", std::env::current_dir());
    let connection = match Connection::open("resources/db-cities-states.db")
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to open database: {}", e);
            return Err(e);
        }
    };
    println!("Database connection established.");

    let categories = ["City", "Country", "Plant"];
    let letters = "ABCDE".chars().collect::<Vec<_>>();

    let random_category = categories[rand::thread_rng().gen_range(0..categories.len())];
    let random_letter = letters[rand::thread_rng().gen_range(0..letters.len())];

    let table_name = format!("{}_{}", random_category, random_letter);
    let query = format!("SELECT name FROM {} ORDER BY RANDOM() LIMIT 1", table_name);

    let mut stmt = connection.prepare(&query)?;
    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        let value: String = row.get(0)?;
        return Ok(format!("Random {} ({}): {}", random_category, random_letter, value));
    }

    Ok(format!("No data found in table {}", table_name))
}

fn main() -> iced::Result {
    CitiesStates::run(Settings::default())
}