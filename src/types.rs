pub enum ArgumentType {
    ADD,
    DELETE,
    COMPLETE,
    UKNOWN,
}

pub struct Argument {
    pub task_content: String,
    pub task_tag: String,
    pub task_priority: i8,
}

pub struct Cli {
    pub top_level_arg: ArgumentType,
    pub arg: Argument,
}
