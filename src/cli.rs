use crate::types::{ArgumentType, Cli, CliInput, InputTask};

/// Converts commandline arguments into machine readable format
///
///```bash
///opus add "update excel #work @tomorrow |||"
///```
pub fn parse_cli(args: Vec<String>) -> Cli {
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

    match r.top_level_arg {
        ArgumentType::UNKNOWN => panic!(
            "Unknown Argument '{}', run 'opus help' for more info on command syntax.",
            args[1].to_string()
        ),
        ArgumentType::NOTENOUGH => panic!("Not enough Arguments."),
        _ => (),
    }

    let task: Vec<&str> = args[2].split(" ").collect();

    match r.top_level_arg {
        ArgumentType::DELETE | ArgumentType::LIST | ArgumentType::FINISH => {
            r.input.query = Some(task.join(" "));
        }
        ArgumentType::ADD => {
            let mut arg = InputTask {
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

/// add the given InputTask to the database
pub fn cli_add_task(t: InputTask) {
    if t.title.is_empty() {
        panic!(
            "Task '{:?}' has no title, a task's title is the only required value!",
            t
        );
    }
}

/// remove the task with the given id from the database
pub fn cli_del_task(id: String) {
    unimplemented!();
}

/// finish the task with the given id in the database
pub fn cli_fin_task(id: String) {
    unimplemented!();
}
