//! Opus types

use rusqlite::Connection;
/// User action
#[derive(Debug, Clone, Copy)]
pub enum ArgumentType {
    /// add a new todo
    ADD,
    /// delete a todo
    DELETE,
    /// mark a todo as finished
    FINISH,
    /// list todo matching the query
    LIST,
    /// given argument is unknown
    UNKNOWN,
    /// not enough arguments supplied
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
}

#[derive(Debug)]
pub struct Cli {
    pub top_level_arg: ArgumentType,
    pub input: CliInput,
}

pub struct Database {
    pub con: Connection,
}
