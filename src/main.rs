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
//! opus ls
//! opus ls "#work"   # List all tasks with tag work (#work):
//! opus ls ","       # List all tasks with priority !! (2):
//! opus ls .2        # List task with id 2
//! ```
//!
//! ### Export tasks
//!
//! ```bash
//! opus export json data       # Exports all tasks in a data.json file
//! opus export csv mycsvfile   # mycsvfile.csv
//! opus export tsv data        # data.tsv
//! ```
//!
//! ## Contributing
use std::env;
use std::io::Write;

use cli::*;
use db::{open_db, Database};
use types::{ArgumentType, Task};

use crate::cli::parse_args;
use crate::types::Cli;

mod cli;
mod db;
mod types;
mod util;

mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result: Cli = parse_args(args);

    let db: Database = open_db();
    db.create_table_if_missing();

    match &result.top_level_arg {
        ArgumentType::Add => {
            let t: Task = match result.input.task {
                Some(x) => x,
                _ => panic!("Input is malformed"),
            };
            cli_add_task(&db, t);
        }
        ArgumentType::List => {
            let query = result.input.query.unwrap();
            let tasks = cli_get_tasks(&db, query.clone());
            println!("{:#?}", &tasks);
            println!("--");
            println!(
                "TODO: {} tasks found matching query: '{}'",
                tasks.len(),
                query
            );
        }
        ArgumentType::Finish => {
            if cli_fin_task(&db, result.input.query.unwrap()) {
                println!("marked task as finished");
            } else {
                println!("marking task as finished failed");
            }
        }
        ArgumentType::Clear => {
            if cli_clear(&db) {
                println!("removed all tasks from database");
            } else {
                println!("couldn't remove all tasks from the database");
            }
        }
        ArgumentType::Export {
            export_type,
            file_name,
        } => {
            let data = cli_export(&db, export_type);

            let file_name_with_extension = format!("{}.{}", file_name, export_type);
            let mut file =
                std::fs::File::create(file_name_with_extension).expect("Unable to open file");
            write!(file, "{}", data).expect("Unable to write");
        },
        ArgumentType::Delete => {
            unimplemented!();
        },
        _ => panic!("Unknown argument."),
    }

    db.con.close().expect("Error while closing database");
}
