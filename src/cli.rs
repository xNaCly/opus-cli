use crate::types::{Argument, ArgumentType, Cli};

/// Converts commandline arguments into machine readable format
/// 
///```bash
///opus add "update excel #work @tomorrow |||"
///```
///```rust
/// let result: Cli = Cli { top_level_arg: ADD, arg: Argument { task_content: "update excelsheet", task_tag: "#work", task_priority: 3, task_due: "@tomorrow" } }
///```
pub fn parse_cli(args: Vec<String>) -> Cli {
    let mut r: Cli = Cli {
        top_level_arg: ArgumentType::UNKNOWN,
        arg: Argument {
            task_content: "".to_string(),
            task_tag: "".to_string(),
            task_priority: 0,
            task_due: "".to_string(),
        },
    };
    if args.len() <= 2 {
        r.top_level_arg = ArgumentType::NOTENOUGH;
    } else {
        match args[1].as_str() {
            "add" | "a" => r.top_level_arg = ArgumentType::ADD,
            "finish" | "f" => r.top_level_arg = ArgumentType::FINISH,
            "delete" | "d" => r.top_level_arg = ArgumentType::DELETE,
            _ => r.top_level_arg = ArgumentType::UNKNOWN,
        }
    }

    match r.top_level_arg {
        ArgumentType::UNKNOWN => panic!(
            "Unknown Argument '{}', run 'opus help' for more info on command syntax.",
            args[1].to_string()
        ),
        ArgumentType::NOTENOUGH => panic!("Not enough Arguments."),
        _ => {}
    }

    let task: Vec<&str> = args[2].split(" ").collect();

    let mut i = 0;
    for x in &task {
        match x.chars().nth(0).unwrap() {
            '#' => r.arg.task_tag = x.to_string(),
            '@' => r.arg.task_due = x.to_string(),
            '|' => r.arg.task_priority = x.len(),
            _ => {
                if i != 0 {
                    r.arg.task_content.push_str(" ");
                }
                r.arg.task_content.push_str(&x);
            }
        }
        i += 1;
    }

    return r;
}
