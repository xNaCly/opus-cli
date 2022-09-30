use crate::db::Database;
use crate::types::{ArgumentType, Cli, CliInput, Task};
use chrono::Utc;

/// Converts command line arguments into machine readable format
///
///```bash
///opus add "update excel #work @tomorrow ,,,"
///```
///
///```rust
///Cli {
///    top_level_arg: Add,
///    input: CliInput {
///        task: Some(
///            Task {
///                id: None,
///                title: "update excel",
///                tag: "#work",
///                priority: 3,
///                due: "@tomorrow",
///            },
///        ),
///        query: None,
///    },
///}
///```
///
///```bash
///opus list "#work"
///```
///
///```rust
///&r = Cli {
///    top_level_arg: List,
///    input: CliInput {
///        task: None,
///        query: Some(
///            "#work",
///        ),
///    },
///}
///```
pub fn parse_args(args: Vec<String>) -> Cli {
    let mut r: Cli = Cli {
        top_level_arg: ArgumentType::Unknown,
        input: CliInput {
            task: None,
            query: None,
        },
    };

    r.top_level_arg = match args[1].as_str() {
        "add" | "a" => ArgumentType::Add,
        "finish" | "f" => ArgumentType::Finish,
        "delete" | "d" => ArgumentType::Delete,
        "list" | "l" => ArgumentType::List,
        _ => ArgumentType::Unknown,
    };

    r.top_level_arg = if args.len() <= 2 && r.top_level_arg != ArgumentType::List {
        ArgumentType::Notenough
    } else {
        r.top_level_arg
    };

    let mut task: Vec<&str> = vec![];

    match r.top_level_arg {
        ArgumentType::List => (),
        ArgumentType::Unknown => panic!(
            "Unknown Argument '{}', run 'opus help' for more info on command syntax.",
            args[1]
        ),
        ArgumentType::Notenough => panic!("Not enough Arguments."),
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

/// remove the task with the given id from the database
pub fn cli_del_task(id: String) {
    // todo: delete a task unimplemented!();
}

/// finish the task with the given id in the database
pub fn cli_fin_task(db: &Database, id: String) -> bool {
    db.mark_task_as_finished(id.parse::<usize>().expect("Given id wasn't an integer")) != 0
}

pub fn cli_get_tasks(db: &Database, q: String) -> Vec<Task> {
    db.get_tasks(q.chars().next().expect("Failure in getting task query"), q)
}
