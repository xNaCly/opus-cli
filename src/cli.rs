use crate::types::{Argument, ArgumentType, Cli};

pub fn parse_cli(args: Vec<String>) -> Cli {
    let mut r: Cli = Cli {
        top_level_arg: ArgumentType::UNKNOWN,
        arg: Argument {
            task_content: "".to_string(),
            task_tag: "".to_string(),
            task_priority: -1,
            // day_and_time: Date;
        },
    };
    if args.len() <= 1 {
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

    return r;
}
