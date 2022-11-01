use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    key_definitions: Vec<Key>,
    mode_switch_key: String,
    events: Vec<Event>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Key {
    vk_code: u32,
    hid_code: u32,
    name: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Event {
    key: String,
    actions: Vec<Action>
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
