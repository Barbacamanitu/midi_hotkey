use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HotkeyMap {
    pub hotkeys: HashMap<u8, HotkeyEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotkeyEntry {
    pub outputs: Vec<Note>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub note: u8,
    pub velocity: f32,
}

impl HotkeyMap {
    pub fn from_json(json: &str) -> HotkeyMap {
        let map: HotkeyMap = serde_json::from_str(&json).unwrap();
        map
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
