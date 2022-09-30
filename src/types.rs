//! Opus types
/// User action
#[derive(Debug, Clone, Copy)]
pub enum ArgumentType {
    /// add a new todo
    Add,
    /// delete a todo
    Delete,
    /// mark a todo as finished
    Finish,
    /// list todo matching the query
    List,
    /// given argument is unknown
    Unknown,
    /// not enough arguments supplied
    Notenough,
}

impl std::fmt::Display for ArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArgumentType::Add => write!(f, "ADD"),
            ArgumentType::List => write!(f, "LIST"),
            ArgumentType::Delete => write!(f, "DELETE"),
            ArgumentType::Finish => write!(f, "FINISH"),
            ArgumentType::Unknown => write!(f, "UNKNOWN"),
            ArgumentType::Notenough => write!(f, "NOTENOUGH"),
        }
    }
}

#[derive(Debug)]
pub struct CliInput {
    pub task: Option<Task>,
    pub query: Option<String>,
}

#[derive(Debug)]
pub struct Task {
    pub id: Option<usize>,
    pub title: String,
    pub tag: String,
    pub priority: usize,
    pub due: String,
    pub finished: bool,
}

#[derive(Debug)]
pub struct Cli {
    pub top_level_arg: ArgumentType,
    pub input: CliInput,
}
