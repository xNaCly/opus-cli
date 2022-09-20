use std::env::consts::OS;
use std::env::var;

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

    let mut path_prefix = match OS {
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
    path_prefix.push_str(CONFIG_PATH);
    return path_prefix;
}