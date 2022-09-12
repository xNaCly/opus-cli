use crate::types::{ArgumentType, Cli, CliInput, Task};
use chrono::Utc;

/// Converts commandline arguments into machine readable format
///
///```bash
///opus add "update excel #work @tomorrow |||"
///```
pub fn parse_args(args: Vec<String>) -> Cli {
    let mut r: Cli = Cli {
        top_level_arg: ArgumentType::UNKNOWN,
        input: CliInput {
            task: None,
            query: None,
        },
    };

    if args.len() <= 2 {
        r.top_level_arg = ArgumentType::NOTENOUGH;
    } else {
        match args[1].as_str() {
            "add" | "a" => r.top_level_arg = ArgumentType::ADD,
            "finish" | "f" => r.top_level_arg = ArgumentType::FINISH,
            "delete" | "d" => r.top_level_arg = ArgumentType::DELETE,
            "list" | "l" => r.top_level_arg = ArgumentType::LIST,
            _ => r.top_level_arg = ArgumentType::UNKNOWN,
        }
    }

    let mut task: Vec<&str> = vec![];

    match r.top_level_arg {
        ArgumentType::LIST => (),
        ArgumentType::UNKNOWN => panic!(
            "Unknown Argument '{}', run 'opus help' for more info on command syntax.",
            args[1].to_string()
        ),
        ArgumentType::NOTENOUGH => panic!("Not enough Arguments."),
        _ => task = args[2].trim().split(" ").collect(),
    }

    match r.top_level_arg {
        ArgumentType::DELETE | ArgumentType::LIST | ArgumentType::FINISH => {
            r.input.query = Some(args.join(" "));
            dbg!(&r.input.query);
        }
        ArgumentType::ADD => {
            let mut arg = Task {
                title: "".to_string(),
                tag: "".to_string(),
                priority: 0,
                due: "".to_string(),
            };
            let mut i = 0;
            for x in &task {
                match x.chars().nth(0).unwrap() {
                    '#' => arg.tag = x.to_string(),
                    '@' => arg.due = x.to_string(),
                    '|' => arg.priority = x.len(),
                    _ => {
                        if i != 0 {
                            arg.title.push_str(" ");
                        }
                        arg.title.push_str(&x);
                    }
                }
                i += 1;
            }
            r.input.task = Some(arg);
        }
        _ => (),
    }
    return r;
}

/// add the given Task to the database
pub fn cli_add_task(mut t: Task) {
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
        title: t.title,
        tag: t.tag,
        priority: t.priority,
        due: t.due,
    };
    // todo: insert into database
    unimplemented!()
}

/// remove the task with the given id from the database
pub fn cli_del_task(id: String) {
    // todo: delete a task
    unimplemented!();
}

/// finish the task with the given id in the database
pub fn cli_fin_task(id: String) {
    // todo: set a task as finished / completed
    unimplemented!();
}

/// get tasks
pub fn cli_get_tasks(_q: String) {
    // todo: get all task from database
    unimplemented!()
}
