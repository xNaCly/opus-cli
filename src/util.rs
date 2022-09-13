use std::env::consts::OS;
use std::env::var;
use std::fs::{File, create_dir_all};
use std::path::Path;

const CONFIG_PATH: &str = "/opus/opus.db";

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
pub fn get_db_path() -> String {
    let opus_path = match var("OPUS_PATH") {
        Ok(r) => r,
        Err(e) => "".to_string(),
    };

    if !opus_path.is_empty() {
        return opus_path;
    };

    let mut prefix = match OS {
        "linux" => match var("XDG_CONFIG_HOME") {
            Ok(r) => r,
            Err(e) => var("HOME")
                .expect("$HOME variable not set, is your operating system configured correctly? Try setting the $OPUS_PATH env variable to a path which opus can access."),
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

pub fn create_file(path: &String) {
    let new_path = Path::new(path);
    let parent = new_path.parent().expect("Couldn't get Parent directory while creating database directory");
    create_dir_all(parent).expect("Couldn't create database directory");
    File::create(new_path).expect("Coudln't create database file");
}
