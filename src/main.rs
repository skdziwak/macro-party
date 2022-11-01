use std::fs;
use gamesense::client::GameSenseClient;
use crate::data::Config;
use crate::handler::KeyboardEventsHandler;
use crate::low_level_handler::EventHandler;

mod low_level_handler;
mod data;
mod handler;


fn main() {
    let result = fs::read_to_string("config.json").expect("Cannot read config.json");
    let config: Config = serde_json::from_str(result.as_str()).expect("Cannot deserialize config");

    let mut game_sense = GameSenseClient::new("MACRO_PARTY", "Macro party", "skdziwak", None)
        .expect("Cannot connect to GameSense");

    let handler = KeyboardEventsHandler::new(game_sense, config).expect("Error occurred during creating keyboard events handler");
    let handler_box: Box<dyn EventHandler> = Box::from(handler);
    low_level_handler::run(handler_box);
}
