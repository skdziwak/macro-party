use std::collections::HashMap;
use gamesense::client::GameSenseClient;
use crate::{Config, EventHandler};
use std::error::Error;
use std::thread;
use std::time::Duration;
use gamesense::handler::Handler;
use serde::Serialize;
use crate::data::{Action, ActionType, Color, Key};
use crate::low_level_handler::VkCode;
use crate::robot;

#[derive(Serialize)]
struct GameSenseColorHandler {
    #[serde(rename = "device-type")]
    device_type: String,
    #[serde(rename = "custom-zone-keys")]
    custom_zone_keys: Vec<u32>,
    color: GameSenseColor,
    mode: String
}

#[derive(Serialize)]
struct GameSenseColor {
    gradient: GameSenseColorGradient
}

#[derive(Serialize)]
struct GameSenseColorGradient {
    zero: Color,
    hundred: Color
}

pub struct KeyboardEventsHandler {
    game_sense: GameSenseClient,
    mode_switch_vk: VkCode,
    enabled: bool,
    events_map: std::sync::Arc<HashMap<VkCode, Vec<Action>>>,
    key_map: std::sync::Arc<HashMap<String, VkCode>>
}

impl Handler for GameSenseColorHandler {

}

impl KeyboardEventsHandler {
    pub fn new(mut game_sense: GameSenseClient, config: Config) -> Result<Self, Box<dyn Error>> {
        game_sense.start_heartbeat();
        let mut keys_by_names: HashMap<&str, &Key> = HashMap::new();
        let mut key_map: HashMap<String, VkCode> = HashMap::new();

        for key in config.key_definitions() {
            keys_by_names.insert(key.name(), key);
            key_map.insert(key.name().to_string(), key.vk_code());
        }
        let mode_switch_key = keys_by_names.get(config.mode_switch_key()).expect("Cannot find mode switch key");
        let mode_switch_vk = mode_switch_key.vk_code();
        game_sense.bind_event("INDICATOR", None, None, None, None, vec![
            GameSenseColorHandler {
                device_type: "keyboard".to_string(),
                custom_zone_keys: vec![
                    mode_switch_key.hid_code()
                ],
                color: GameSenseColor {
                    gradient: GameSenseColorGradient {
                        zero: config.disabled_indicator_color().clone(),
                        hundred: config.indicator_color().clone()
                    }
                },
                mode: "color".to_string()
            }
        ])?;

        let mut event_hid_codes: Vec<u32> = Vec::new();
        let mut events_map: HashMap<VkCode, Vec<Action>> = HashMap::new();

        for event in config.events() {
            let key = keys_by_names.get(event.key()).expect("Cannot find key");
            event_hid_codes.push(key.hid_code());

            events_map.insert(key.vk_code(), event.actions().clone());
        }

        if !event_hid_codes.is_empty() {
            game_sense.bind_event("EVENTS", None, None, None, None, vec![
                GameSenseColorHandler {
                    device_type: "keyboard".to_string(),
                    custom_zone_keys: event_hid_codes,
                    color: GameSenseColor {
                        gradient: GameSenseColorGradient {
                            zero: config.background_color().clone(),
                            hundred: config.macros_color().clone()
                        }
                    },
                    mode: "color".to_string()
                }
            ])?;
            game_sense.trigger_event("EVENTS", 0)?;
        }
        game_sense.trigger_event("INDICATOR", 0)?;

        Ok(Self {
            game_sense,
            mode_switch_vk,
            enabled: false,
            events_map: std::sync::Arc::new(events_map),
            key_map: std::sync::Arc::new(key_map)
        })
    }

    fn execute_actions(&self, code: VkCode) {
        let events_map_arc = self.events_map.clone();
        let keymap_arc = self.key_map.clone();
        thread::spawn(move || {
            let actions = events_map_arc.get(&code).expect("Cannot find actions");
            for action in actions {
                if execute_action(action, keymap_arc.as_ref()).is_none() {
                    println!("WARN: Action interrupted. One of required fields is empty.");
                    break;
                }
            }
        });
    }
}

fn execute_action(action: &Action, keymap: &HashMap<String, VkCode>) -> Option<()> {
    match action.action_type() {
        ActionType::Wait => {
            thread::sleep(Duration::from_millis(action.duration()?))
        }
        ActionType::LeftClick => {
            robot::left_click(action.x()?, action.y()?);
        }
        ActionType::RightClick => {
            robot::right_click(action.x()?, action.y()?);
        }
        ActionType::Key => {
            let key = action.key().as_ref()?;
            let code = keymap.get(key)?;
            robot::press_key(code.clone());
            robot::release_key(code.clone());
        }
        ActionType::KeyDown => {
            let key = action.key().as_ref()?;
            let code = keymap.get(key)?;
            robot::press_key(code.clone());
        }
        ActionType::KeyUp => {
            let key = action.key().as_ref()?;
            let code = keymap.get(key)?;
            robot::release_key(code.clone());
        }
    }
    Some(())
}

impl Drop for KeyboardEventsHandler {
    fn drop(&mut self) {
        self.game_sense.stop_heartbeat().expect("Cannot stop heartbeat");
    }
}


impl EventHandler for KeyboardEventsHandler {
    fn key_pressed(&mut self, code: VkCode) -> bool {
        if code == self.mode_switch_vk {
            match self.enabled {
                true => {
                    self.game_sense.trigger_event("INDICATOR", 0).expect("Cannot send keyboard event");
                    self.game_sense.trigger_event("EVENTS", 0).expect("Cannot send keyboard event");
                    self.enabled = false;
                    println!("Disabled macro mode");
                }
                false => {
                    self.game_sense.trigger_event("INDICATOR", 100).expect("Cannot send keyboard event");
                    self.game_sense.trigger_event("EVENTS", 100).expect("Cannot send keyboard event");
                    self.enabled = true;
                    println!("Enabled macro mode");
                }
            }
            return true;
        }
        if self.enabled && self.events_map.contains_key(&code) {
            return true;
        }
        return false;
    }

    fn key_released(&mut self, code: VkCode) -> bool {
        if code == self.mode_switch_vk {
            return true;
        }
        if self.enabled && self.events_map.contains_key(&code) {
            self.execute_actions(code);
            return true;
        }
        return false;
    }
}
