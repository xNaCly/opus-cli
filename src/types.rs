//! Opus types
use std::fmt;

use chrono::Utc;
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

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub struct Task {
    pub id: Option<usize>,
    pub title: String,
    pub tag: String,
    pub priority: usize,
    pub due: String,
    pub finished: bool,
}

impl From<String> for Task {
    /// This method is intended to parse tasks from cli input
    /// - words prefixed with `#` are considered tags, only the last found task is kept as the tasks tag
    /// - words prefixed with `@` are considered dates, '@tomorrow' and '@today' will be replaced with the corresponding dates in the 'yyyy-MM-dd' format
    /// - a number prefixed with `.` is considered a priority
    fn from(item: String) -> Self {
        let mut t = Task {
            title: "".to_string(),
            id: None,
            tag: "".to_string(),
            priority: 0,
            due: "".to_string(),
            finished: false,
        };
        let tokens: Vec<&str> = item.split(' ').collect();
        for (i, token) in tokens.iter().enumerate() {
            match token.chars().next() {
                Some('#') => t.tag = token.to_string(),
                Some('@') => {
                    t.due = match token {
                        &"@tomorrow" => Utc::now().date().succ().format("%Y-%m-%d").to_string(),
                        &"@today" => Utc::now().format("%Y-%m-%d").to_string(),
                        _ => token[1..].to_string(),
                    }
                }
                Some('.') => {
                    t.priority = token[1..]
                        .to_string()
                        .parse::<usize>()
                        .expect("Priority not a number")
                }
                _ => {
                    if i != 0 {
                        t.title.push(' ')
                    }
                    t.title.push_str(token);
                }
            }
        }
        t
    }
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
