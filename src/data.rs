use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    key_definitions: Vec<Key>,
    mode_switch_key: String,
    events: Vec<Event>
}

impl Config {
    pub fn key_definitions(&self) -> &Vec<Key> {
        &self.key_definitions
    }
    pub fn mode_switch_key(&self) -> &str {
        &self.mode_switch_key
    }
    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Key {
    vk_code: u32,
    hid_code: u32,
    name: String
}

impl Key {
    pub fn vk_code(&self) -> u32 {
        self.vk_code
    }
    pub fn hid_code(&self) -> u32 {
        self.hid_code
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Event {
    key: String,
    actions: Vec<Action>
}

impl Event {
    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn actions(&self) -> &Vec<Action> {
        &self.actions
    }
}

#[derive(Serialize, Deserialize)]
pub enum ActionType {
    Wait,
    Click,
    Key,
    KeyDown,
    KeyUp
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Action {
    action_type: ActionType,
    duration: Option<i32>,
    key: Option<String>,
    x: Option<i32>,
    y: Option<i32>
}

impl Action {
    pub fn action_type(&self) -> &ActionType {
        &self.action_type
    }
    pub fn duration(&self) -> Option<i32> {
        self.duration
    }
    pub fn key(&self) -> &Option<String> {
        &self.key
    }
    pub fn x(&self) -> Option<i32> {
        self.x
    }
    pub fn y(&self) -> Option<i32> {
        self.y
    }
}
