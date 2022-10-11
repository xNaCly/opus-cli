//! Opus types
use std::fmt;

use serde::Serialize;
/// User action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentType {
    /// add a new todo
    Add,
    /// delete a todo
    Delete,
    /// remove all tasks
    Clear,
    /// mark a todo as finished
    Finish,
    /// list todo matching the query
    List,
    /// given argument is unknown
    Unknown,
    /// not enough arguments supplied
    Notenough,
    Export {
        export_type: ExportType,
        file_name: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportType {
    Json,
    Csv,
    Tsv,
}

impl fmt::Display for ExportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportType::Json => write!(f, "json"),
            ExportType::Csv => write!(f, "csv"),
            ExportType::Tsv => write!(f, "tsv"),
        }
    }
}

#[derive(Debug)]
pub struct CliInput {
    pub task: Option<Task>,
    pub query: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Task {
    pub id: Option<usize>,
    pub title: String,
    pub tag: String,
    pub priority: usize,
    pub due: String,
    pub finished: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = self.id.expect("id not found, task faulty");
        write!(f, "[{}]: '{}'", id, self.title)?;
        let tag = match self.tag.as_str() {
            "tag" => "",
            _ => &self.tag,
        };
        let due = match self.due.as_str() {
            "due" => "",
            _ => &self.due,
        };

        if !due.is_empty() {
            write!(f, " ({})", due)?;
        }
        if !tag.is_empty() {
            write!(f, " {}", tag)?;
        }
        if self.priority != 0 {
            let mut prio: String = String::new();
            for i in 0..self.priority {
                prio.push(',');
            }
            write!(f, " [{}]", prio)?;
        }
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct Cli {
    pub top_level_arg: ArgumentType,
    pub input: CliInput,
}
