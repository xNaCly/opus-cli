use clap::{arg, command, crate_authors, crate_description, Arg, ArgAction, Command};
use cli::{cli_add_task, cli_clear, cli_del_task, cli_export, cli_fin_task, cli_get_tasks};
use db::{open_db, Database};
use std::io::Write;
use std::{env, fs::File};
use types::{ExportType, Task};

use crate::types::{SortMode, SortOrder};

mod cli;
mod db;
mod types;
mod util;

mod tests;

fn main() {
    // INFO: documentation clap: https://docs.rs/clap/latest/clap/_tutorial/index.html#subcommands
    let commands =
        command!()
            .about(crate_description!())
            .author(crate_authors!("\n"))
            .propagate_version(true)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new("add")
                    .visible_alias("a")
                    .about("create a new task")
                    .arg(arg!(<CONTENT>)),
            )
            .subcommand(
                Command::new("delete")
                    .visible_aliases(["del", "d"])
                    .about("delete a task with the given id")
                    .arg(arg!(<ID>)),
            )
            .subcommand(Command::new("clear").about("remove all tasks from the database"))
            .subcommand(
                Command::new("finish")
                    .visible_aliases(["fin", "f"])
                    .about("mark the task with the given id as finished")
                    .arg(arg!(<ID>)),
            )
            .subcommand(
                Command::new("list")
                    .visible_aliases(["ls", "l"])
                    .about("list tasks matching the given query")
                    .arg(arg!([QUERY]))
                    .arg(Arg::new("sort_by").long("sort_by").help(
                        "sort tasks by given param: (id, due, finished, title, priority, tag)",
                    ))
                    .arg(Arg::new("sort_order").long("sort_order").help(
                        "sort tasks by given param: (id, due, finished, title, priority, tag)",
                    ))
                    .arg(
                        // INFO: documentation for flags: https://docs.rs/clap/latest/clap/_tutorial/index.html#flags
                        Arg::new("finished")
                            .short('f')
                            .long("finished")
                            .action(ArgAction::SetTrue)
                            .help("displays tasks marked as finished"),
                    ),
            )
            .subcommand(
                Command::new("export")
                    .about("export all tasks")
                    .arg(
                        Arg::new("fileformat")
                            .long("format")
                            .short('f')
                            .required(true)
                            .help("select the export format: json or csv"),
                    )
                    .arg(
                        Arg::new("filename")
                            .long("output")
                            .short('o')
                            .required(true)
                            .help("select the filename for the export"),
                    ),
            )
            .get_matches();

    let db: Database = open_db();
    db.create_table_if_missing();

    match commands.subcommand() {
        Some(("list", sub_matches)) => {
            let display_finished = sub_matches.get_flag("finished");
            let default_value = &String::from("list");
            let sort_by = match sub_matches.get_one::<String>("sort_by") {
                Some(e) => SortMode::from(&e[..]),
                _ => SortMode::NoSort,
            };
            let sort_order = match sub_matches.get_one::<String>("sort_order") {
                Some(e) => SortOrder::from(&e[..]),
                _ => SortOrder::ASC,
            };
            let query = sub_matches
                .get_one::<String>("QUERY")
                .unwrap_or(default_value);
            let tasks = cli_get_tasks(
                &db,
                query.to_string(),
                display_finished,
                sort_by,
                sort_order,
            );
            for t in &tasks {
                println!("{}", t);
            }
            println!(
                "--\n{} tasks found matching query: '{}'",
                tasks.len(),
                query
            );
        }
        Some(("add", sub_matches)) => {
            let t: Task = Task::from(
                *sub_matches
                    .get_one::<&str>("CONTENT")
                    .expect("Failure in parsing task"),
            );

            cli_add_task(&db, t);
        }
        Some(("delete", sub_matches)) => {
            let id = sub_matches
                .get_one::<String>("ID")
                .expect("Couldn't get id from input")
                .to_string();

            let feedback = cli_del_task(&db, id);
            println!(
                "{}",
                if feedback {
                    "deleted task"
                } else {
                    "failed to delete given task"
                }
            )
        }
        Some(("finish", sub_matches)) => {
            let id = sub_matches
                .get_one::<String>("ID")
                .expect("Couldn't get id from input")
                .to_string();

            let feedback = cli_fin_task(&db, id);
            println!(
                "{}",
                if feedback {
                    "marked task as finished"
                } else {
                    "failed to mark task as finished"
                }
            )
        }
        Some(("clear", _)) => {
            let feedback = cli_clear(&db);
            println!(
                "{}",
                if feedback {
                    "cleared database"
                } else {
                    "failed to clear database"
                }
            );
        }
        Some(("export", sub_matches)) => {
            let temp = sub_matches
                .get_one::<String>("fileformat")
                .expect("Bad Fileformat");

            let export_type = ExportType::from(&temp[..]);

            let filename = sub_matches
                .get_one::<String>("filename")
                .expect("Bad Filename");
            let data = cli_export(&db, &export_type);
            let mut file = File::create(filename).expect("Unable to export to file");
            write!(file, "{}", data).expect("Unable to write data to file");
        }
        _ => (),
    }

    db.con.close().expect("Error while closing database");
}
