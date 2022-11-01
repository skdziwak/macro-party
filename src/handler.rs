use gamesense::client::GameSenseClient;
use crate::{Config, EventHandler, low_level_handler};

pub struct Handler {
    game_sense: GameSenseClient,
    config: Config
}

impl Handler {
    pub fn new(mut game_sense: GameSenseClient, mut config: Config) -> Self {
        game_sense.start_heartbeat();
        Self { game_sense, config }
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        self.game_sense.stop_heartbeat().expect("Cannot stop heartbeat");
    }
}

impl EventHandler for Handler {
    fn key_pressed(&self, code: low_level_handler::VkCode) -> bool {
        println!("Pressed {0}", code);
        return true;
    }

    fn key_released(&self, code: low_level_handler::VkCode) -> bool {
        println!("Released {0}", code);
        return false;
    }
}
