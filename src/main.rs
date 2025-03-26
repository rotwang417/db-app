mod views;
mod menu;
mod game_state;
mod messages;
mod cities_states;
mod database;

use menu::CitiesStates;

use iced::{executor, Settings};
use iced::{Application, Command, Element, Length};
use iced::widget::{Button, button, Column, Container, Text};

use crate::views::{MenuOption, View};

fn main() -> iced::Result
{
    CitiesStates::run(Settings::default())
}