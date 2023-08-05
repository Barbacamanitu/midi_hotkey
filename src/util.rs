use std::{fs, path::PathBuf};

pub fn get_vst_path() -> PathBuf {
    process_path::get_dylib_path()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}

pub fn get_json_path() -> PathBuf {
    get_vst_path().join("midi_hotkey.json")
}

pub fn read_json_file() -> String {
    fs::read_to_string(get_json_path()).unwrap()
}
