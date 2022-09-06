use crate::types::Task;
use std::env::consts::OS;
use std::path::Path;

const FILE_NAME: &str = "opus_todo.txt";

fn get_config_path() -> String {
    return match OS {
        "linux" => "$XDG_CONFIG_HOME/opus",
        "windows" => "%LOCALAPPDATA%/opus",
        _ => ".opus/",
    }
    .to_string();
}

fn does_config_exist() -> bool {
    let mut config_path = get_config_path();
    config_path.push_str(FILE_NAME);
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
