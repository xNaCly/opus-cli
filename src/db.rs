//! Opus database wrapper
//!
//! Opus follows the unix principle and therefore stores all its data in a text based csv database.
use crate::types::Task;
use std::env::consts::OS;
use std::env::var;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const CONFIG_PATH: &str = "/opus/opus_todo.txt";

/// Get system dependent path to config files
///
/// Opus amis to be platform indepentent, thus its database is located at:
/// - `$OPUS_PATH ` or `%OPUS_PATH%`
/// - windows: `%LOCALAPPDATA%/opus/opus_todo.txt`
/// - linux: `$XDG_CONFIG_HOME/opus/opus_todo.txt` or `$HOME/opus/opus_todo.txt`
/// - anything else uses the root directory (may require admin permission on windows and root on unix)
///
/// Testing status:
/// - windows ✅
/// - linux ❌
/// - macos ❌
pub fn get_config_path() -> String {
    // let opus_path = match var("OPUS_PATH") {
    //     Ok(r) => r,
    //     Err(e) => "".to_string(),
    // };
    let opus_path = "c:/Users/gro112741/Desktop/opus_todo.txt".to_string();

    if !opus_path.is_empty() {
        return opus_path;
    };

    let mut prefix = match OS {
        "linux" => match var("XDG_CONFIG_HOME") {
            Ok(r) => r,
            Err(e) => var("HOME")
                .expect("$HOME variable not set, is your operating system configured correctly?"),
        },
        "windows" => match var("LOCALAPPDATA") {
            Ok(r) => r,
            Err(e) => "/".to_string(),
        },
        _ => "/".to_string(),
    }
    .to_string();
    prefix.push_str(CONFIG_PATH);
    return prefix;
}

pub fn does_file_exist(path: &String) -> bool {
    Path::new(path).exists()
}

pub fn db_add(t: Task) {
    let path = get_config_path();
    if !does_file_exist(&path) {
        panic!("Can't find database file")
    }
}
pub fn db_remove(id: usize) {
    unimplemented!()
}
pub fn db_finish(id: usize) {
    unimplemented!()
}

pub fn db_get() {
    let path = get_config_path();
    if !does_file_exist(&path) {
        panic!("Can't find database file")
    }
    dbg!(&path);
    let file = File::open(path).expect("Couldn't open database file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
