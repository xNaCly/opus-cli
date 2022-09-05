#[derive(Debug, Clone, Copy)]
pub enum ArgumentType {
    ADD,
    DELETE,
    FINISH,
    LIST,
    UNKNOWN,
    NOTENOUGH,
}

impl std::fmt::Display for ArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArgumentType::ADD => write!(f, "ADD"),
            ArgumentType::LIST => write!(f, "LIST"),
            ArgumentType::DELETE => write!(f, "DELETE"),
            ArgumentType::FINISH => write!(f, "FINISH"),
            ArgumentType::UNKNOWN => write!(f, "UNKNOWN"),
            ArgumentType::NOTENOUGH => write!(f, "NOTENOUGH"),
        }
    }
}

#[derive(Debug)]
pub struct InputTask {
    pub title: String,
    pub tag: String,
    pub priority: usize,
    pub due: String,
}

#[derive(Debug)]
pub struct CliInput {
    pub task: Option<InputTask>,
    pub query: Option<String>,
}

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub tag: String,
    pub priority: usize,
    pub due: String,
    pub id: usize,
}

#[derive(Debug)]
pub struct Cli {
    pub top_level_arg: ArgumentType,
    pub input: CliInput,
}
