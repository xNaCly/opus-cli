#![allow(dead_code)]
#![allow(unused_variables)]
//! # opusCli
//!
//! cli todo tool - add, edit, tag and delete tasks
//!
//! ## Showcase
//!
//! ## Installation
//!
//! ## Syntax
//!
//! ### Add a task
//!
//! ```
//! opus add "update excelsheet #work @tomorrow ,,,"
//!           ^^^^^^^^^^^^^^^^^ ^^^^^ ^^^^^^^^^ ^^^
//!           |                 |     |         |
//!           |                 |     |         |_ priority
//!           |                 |     |
//!           |                 |     |_ due date
//!           |                 |
//!           |                 |_ tag
//!           |_ title
//! ```
//!
//! -   tags are prefixed with a `#` and should contain `_` instead of spaces.
//! -   due date is prefixed with a `@` and can either be `@today`, `@tomorrow` or a date (`yyyy-mm-dd`)
//! -   priority is specified using `,` - highest priority is one `,`
//!
//! ### List tasks
//!
//! ```bash
//! opus list
//! opus list "#work"   # List all tasks with tag work (#work):
//! opus list ","       # List all tasks with priority !! (2):
//! opus list .2        # List task with id 2
//! ```
//!
//! ## Contributing
use std::env;

use cli::*;
use types::{ArgumentType, Task};

use crate::cli::parse_args;
use crate::types::Cli;

mod cli;
mod db;
mod types;

mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result: Cli = parse_args(args);
    match &result.top_level_arg {
        ArgumentType::ADD => {
            let t: Task = match result.input.task {
                Some(x) => x,
                _ => panic!("Input is malformated"),
            };
            cli_add_task(t);
        }
        ArgumentType::LIST => {
            cli_get_tasks("".to_string());
        }
        _ => panic!("Unkown argument."),
    }
}
