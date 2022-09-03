#[derive(Debug, Clone, Copy)]
pub enum ArgumentType {
    ADD,
    DELETE,
    FINISH,
    UNKNOWN,
    NOTENOUGH,
}

impl std::fmt::Display for ArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArgumentType::ADD => write!(f, "ADD"),
            ArgumentType::DELETE => write!(f, "DELETE"),
            ArgumentType::FINISH => write!(f, "FINISH"),
            ArgumentType::UNKNOWN => write!(f, "UNKNOWN"),
            ArgumentType::NOTENOUGH => write!(f, "NOTENOUGH"),
        }
    }
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
