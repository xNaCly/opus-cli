use crate::types::{Argument, ArgumentType, Cli};

pub fn parse_cli(args: Vec<String>) -> Cli {
    let mut r: Cli = Cli {
        top_level_arg: ArgumentType::UKNOWN,
        arg: Argument {
            task_content: "".to_string(),
            task_tag: "".to_string(),
            task_priority: -1,
        },
    };
    match args[1].as_str() {
        "add" => {
            r.top_level_arg = ArgumentType::ADD;
        }
        "finish" => {
            r.top_level_arg = ArgumentType::COMPLETE;
        }
        "delete" => {
            r.top_level_arg = ArgumentType::DELETE;
        }
        _ => panic!(
            "Unknown Argument '{}', run 'opus help' for more info on command syntax",
            args[1].to_string()
        ),
    }
    return r;
}
