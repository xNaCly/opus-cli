use crate::db::Database;
use crate::types::{ArgumentType, Cli, CliInput, ExportType, Task};
use chrono::Utc;

pub fn parse_args(args: Vec<String>) -> Cli {
    let mut r: Cli = Cli {
        top_level_arg: ArgumentType::Unknown,
        input: CliInput {
            task: None,
            query: None,
        },
    };

    if args.len() == 1 {
        r.top_level_arg = ArgumentType::List;
        r.input.query = Some("list".to_string());
        return r;
    }

    r.top_level_arg = match args[1].as_str() {
        "help" | "h" => {
            std::process::exit(1);
        }
        "add" | "a" => ArgumentType::Add,
        "finish" | "f" => ArgumentType::Finish,
        "delete" | "d" => ArgumentType::Delete,
        "list" | "l" => ArgumentType::List,
        "clear" => ArgumentType::Clear,
        "export" => {
            if args.len() <= 3 {
                panic!("Not enough args for export")
            }

            let file_name = args[3].to_lowercase();
            match args[2].to_lowercase().as_str() {
                "json" => ArgumentType::Export {
                    export_type: ExportType::Json,
                    file_name,
                },
                "csv" => ArgumentType::Export {
                    export_type: ExportType::Csv,
                    file_name,
                },
                "tsv" => ArgumentType::Export {
                    export_type: ExportType::Tsv,
                    file_name,
                },
                _ => panic!("Unknown format \"{}\"", args[2].to_lowercase()),
            }
        }
        _ => ArgumentType::Unknown,
    };

    r.top_level_arg = if (args.len() <= 2
        && r.top_level_arg != ArgumentType::List
        && r.top_level_arg != ArgumentType::Clear)
        || (args.len() <= 3
            && matches!(
                r.top_level_arg,
                ArgumentType::Export {
                    export_type: _,
                    file_name: _
                }
            )) {
        ArgumentType::Notenough
    } else {
        r.top_level_arg
    };

    let mut task: Vec<&str> = vec![];

    match &r.top_level_arg {
        ArgumentType::List => (),
        ArgumentType::Clear => (),
        ArgumentType::Export {
            export_type: _,
            file_name: file,
        } => (),
        ArgumentType::Unknown => panic!(
            "Unknown Argument '{}', run 'opus help' for more info on command syntax.",
            args[1]
        ),
        ArgumentType::Notenough => {
            panic!("Not enough Arguments.");
        }
        _ => task = args[2].trim().split(' ').collect(),
    }

    match r.top_level_arg {
        ArgumentType::Delete | ArgumentType::List | ArgumentType::Finish => {
            r.input.query = Some(args.last().expect("Not enough Arguments.").to_string());
        }
        ArgumentType::Add => {
            let mut arg = Task {
                id: None,
                title: "".to_string(),
                tag: "".to_string(),
                priority: 0,
                due: "".to_string(),
                finished: false,
            };
            for (i, x) in task.iter().enumerate() {
                match x.chars().next().unwrap() {
                    '#' => arg.tag = x.to_string(),
                    '@' => arg.due = x.to_string(),
                    ',' => arg.priority = x.len(),
                    _ => {
                        if i != 0 {
                            arg.title.push(' ');
                        }
                        arg.title.push_str(x);
                    }
                }
            }
            r.input.task = Some(arg);
        }
        _ => (),
    }
    r
}

/// add the given Task to the database
pub fn cli_add_task(db: &Database, mut t: Task) {
    if t.title.is_empty() {
        panic!(
            "Task '{:?}' has no title, a task's title is the only required value!",
            t
        );
    }

    if !t.due.is_empty() {
        t.due = match &t.due[..] {
            "@tomorrow" => Utc::now().date().succ().format("%Y-%m-%d").to_string(),
            "@today" => Utc::now().format("%Y-%m-%d").to_string(),
            _ => t.due,
        }
    }
    let task = Task {
        id: None,
        title: t.title,
        tag: t.tag,
        priority: t.priority,
        due: t.due,
        finished: false,
    };

    db.insert_task(task);
}

pub fn cli_del_task(db: &Database, id: String) -> bool {
    db.delete_task(id.parse::<usize>().expect("Given id wasn't an integer")) != 0
}

pub fn cli_fin_task(db: &Database, id: String) -> bool {
    db.mark_task_as_finished(id.parse::<usize>().expect("Given id wasn't an integer")) != 0
}

pub fn cli_get_tasks(db: &Database, q: String) -> Vec<Task> {
    db.get_tasks(q.chars().next().expect("Failure in getting task query"), q)
}

pub fn cli_clear(db: &Database) -> bool {
    db.clear_all_tasks() != 0
}

pub fn cli_export(db: &Database, export_type: &ExportType) -> String {
    db.export(export_type)
}
