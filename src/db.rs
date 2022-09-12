// todo: Thinking about the implementation of a database:
//! # Thinking about the implementation of a database:
//!
//! maybe switch from a file based approch to a sqlite database, the currently planned implementation has the following flaws, which could be removed by using a relational database instead of a file:
//!
//! - deleting a row would cause the program to read the whole file, remove the row and write the whole file minus the row back to disk (this is a waste of ressources). Using sql one can just `DELETE` an entry
//!
//! - to keep tabs on the current id, the program needs to read all lines from the disk and increment the last id. Sqlite autoincrements the primary id.
//!
//! - `opus list` requires a query engine, which is already included in sql, but not in this program
//!
//!
//! # Opus database wrapper
use crate::types::Task;
use std::env::consts::OS;
use std::env::var;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
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
pub fn get_db_path() -> String {
    // todo: uncomment this after testing
    // let opus_path = match var("OPUS_PATH") {
    //     Ok(r) => r,
    //     Err(e) => "".to_string(),
    // };
    // todo: remove this after testing
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

pub fn create_new_db() {
    let path = get_db_path();
}
