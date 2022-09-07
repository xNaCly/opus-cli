use crate::types::Task;
use std::env::consts::OS;
use std::env::var;
use std::path::Path;

const CONFIG_PATH: &str = "/opus/opus_todo.txt";

/// Get system dependent path to config files
///
/// Testing status:
/// - windows ✅
/// - linux ❌
/// - macos ❌
pub fn get_config_path() -> String {
    return match OS {
        "linux" => match var("XDG_CONFIG_HOME") {
            Ok(r) => r,
            Err(e) => var("HOME")
                .expect("$HOME variable not set, is your operating system configured correctly?"),
        },
        "windows" => match var("LOCALAPPDATA") {
            Ok(r) => r,
            Err(e) => "".to_string(),
        },
        _ => match var("OPUS_PATH") {
            Ok(r) => r,
            Err(e) => "".to_string(),
        },
    }
    .to_string();
}

pub fn does_config_exist() -> bool {
    let mut config_path = get_config_path();
    config_path.push_str(CONFIG_PATH);
    Path::new(&config_path).exists()
}

pub fn db_add(t: Task) {
    if !does_config_exist() {
        panic!("Can't find database file")
    }
}
pub fn db_remove(id: usize) {
    unimplemented!()
}
pub fn db_finish(id: usize) {
    unimplemented!()
}

pub fn db_get(q: &str) {
    unimplemented!();
}
