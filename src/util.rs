use std::env::consts::OS;
use std::env::var;
use std::fs::create_dir;
use std::path::Path;

const CONFIG_PATH: &str = "/opus/opus.db";
pub const OPUS_HELP: &str = "Usage: opus [command] [params]
Commands:
    list  \t<query>
    add   \t<task> <tag> <priority> <date>
    finish\t<id>
    delete\t<id>
    export\t<format> <filename>
    clear
";

/// Get system dependent path to config files
///
/// Opus amis to be working on all platforms, thus its database is located at:
/// - `$OPUS_PATH` or `%OPUS_PATH%`
/// - windows: `%LOCALAPPDATA%/opus/opus.db`
/// - linux or macos: `$XDG_CONFIG_HOME/opus/opus.db` or `$HOME/opus/opus.db`
///
/// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
/// <strong>Warning:</strong> If the target system is not in the above, opus will panic. To fix this set the <code>OPUS_PATH</code> env variable. </p>
///
/// Tested on:
/// - windows ✅
/// - linux ✅
/// - macos ✅
pub fn get_db_path() -> String {
    let opus_path = match var("OPUS_PATH") {
        Ok(r) => r,
        Err(_) => "".to_string(),
    };

    let mut path_prefix = if opus_path.is_empty() {
        match OS {
          "linux" | "macos" => match var("XDG_CONFIG_HOME") {
              Ok(r) => r,
              Err(_) => var("HOME")
                  .expect("$HOME variable not set, is your operating system configured correctly? Try setting the $OPUS_PATH env variable to a path which opus can access."),
          },
          "windows" => match var("LOCALAPPDATA") {
              Ok(r) => r,
              Err(_) => "/".to_string(),
          },
          _ => panic!("Couldn't find a config file implementation for your system, consider setting the $OPUS_PATH or %OPUS_PATH% system variable to point opus to the desired config folder - otherwise opus won't work.")
        }
    } else {
        opus_path
    };
    path_prefix.push_str(CONFIG_PATH);
    path_prefix
}

pub fn create_dir_if_not_exist(path: &String) -> bool {
    let path = Path::new(&path);
    let ppath = &path.parent().expect("Couldn't get database path parent");
    if !ppath.exists() {
        create_dir(ppath).is_ok()
    } else {
        true
    }
}
