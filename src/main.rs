use clap::{arg, command, error::Error, Arg, ArgAction, Command};
use cli::{cli_add_task, cli_get_tasks};
use db::{open_db, Database};
use std::env;
use types::Task;

mod cli;
mod db;
mod types;
mod util;

mod tests;

fn main() {
    // INFO: documentation clap: https://docs.rs/clap/latest/clap/_tutorial/index.html#subcommands
    let commands = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("create a new todo")
                .arg(arg!(<CONTENT>)),
        )
        .subcommand(
            Command::new("delete")
                .about("delete the todo with the given id")
                .arg(arg!(<ID>)),
        )
        .subcommand(Command::new("clear").about("remove all tasks from the database"))
        .subcommand(
            Command::new("finish")
                .about("mark the todo with the given id as finished")
                .arg(arg!(<ID>)),
        )
        .subcommand(
            Command::new("list")
                .about("list todos matching the given query")
                .arg(arg!([QUERY]))
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
            let query = sub_matches
                .get_one::<String>("QUERY")
                .unwrap_or(default_value);

            let tasks = cli_get_tasks(&db, query.to_string(), display_finished);
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
                sub_matches
                    .get_one::<String>("CONTENT")
                    .expect("Failure in parsing task")
                    .to_string(),
            );

            cli_add_task(&db, t);
        }
        _ => (),
    }

    db.con.close().expect("Error while closing database");

    // match &result.top_level_arg {
    //     ArgumentType::Add => {
    //         let t: Task = match result.input.task {
    //             Some(x) => x,
    //             _ => panic!("Input is malformed"),
    //         };
    //         cli_add_task(&db, t);
    //     }
    //     ArgumentType::List => {
    //         let query = result.input.query.unwrap();
    //         let tasks = cli_get_tasks(&db, query.clone());
    //         for task in &tasks {
    //             println!("{}", task);
    //         }
    //         println!("--");
    //         println!(
    //             "TODO: {} tasks found matching query: '{}'",
    //             tasks.len(),
    //             query
    //         );
    //     }
    //     ArgumentType::Finish => {
    //         if cli_fin_task(&db, result.input.query.unwrap()) {
    //             println!("marked task as finished");
    //         } else {
    //             println!("marking task as finished failed");
    //         }
    //     }
    //     ArgumentType::Clear => {
    //         if cli_clear(&db) {
    //             println!("removed all tasks from database");
    //         } else {
    //             println!("couldn't remove all tasks from the database");
    //         }
    //     }
    //     ArgumentType::Export {
    //         export_type,
    //         file_name,
    //     } => {
    //         let data = cli_export(&db, export_type);

    //         let file_name_with_extension = format!("{}.{}", file_name, export_type);
    //         let mut file =
    //             std::fs::File::create(file_name_with_extension).expect("Unable to open file");
    //         write!(file, "{}", data).expect("Unable to write");
    //     }
    //     ArgumentType::Delete => {
    //         if cli_del_task(&db, result.input.query.unwrap()) {
    //             println!("deleted task");
    //         } else {
    //             println!("couldn't delete task");
    //         }
    //     }
    //     _ => panic!("Unknown argument."),
    // }
}
