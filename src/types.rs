//! Opus types
use std::fmt::{self};

use chrono::Utc;
use serde::Serialize;
/// User action

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportType {
    Json,
    Csv,
    Tsv,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    Id,
    Due,
    Finished,
    Title,
    Priority,
    Tag,
    NoSort,
}

impl fmt::Display for SortMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Id => "id",
                Self::Due => "due",
                Self::Finished => "finished",
                Self::Title => "title",
                Self::Priority => "priority",
                Self::Tag => "tag",
                Self::NoSort => "",
            }
        )
    }
}

impl From<&str> for SortMode {
    fn from(s: &str) -> Self {
        match s {
            "id" => SortMode::Id,
            "due" => SortMode::Due,
            "finished" => SortMode::Finished,
            "title" => SortMode::Title,
            "priority" => SortMode::Priority,
            "tag" => SortMode::Tag,
            _ => SortMode::NoSort,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    DESC,
    ASC,
}

impl fmt::Display for SortOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DESC => "DESC",
                _ => "ASC",
            }
        )
    }
}

impl From<&str> for SortOrder {
    fn from(s: &str) -> Self {
        match s {
            "desc" => SortOrder::DESC,
            "asc" => SortOrder::ASC,
            _ => panic!("invalid sort direction"),
        }
    }
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

impl From<&str> for ExportType {
    fn from(s: &str) -> Self {
        match s {
            "csv" => ExportType::Csv,
            "tsv" => ExportType::Tsv,
            "json" => ExportType::Json,
            _ => panic!("not a valid export type"),
        }
    }
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

impl From<&str> for Task {
    /// This method is intended to parse tasks from cli input
    /// - words prefixed with `#` are considered tags, only the last found task is kept as the tasks tag
    /// - words prefixed with `@` are considered dates, '@tomorrow' and '@today' will be replaced with the corresponding dates in the 'yyyy-MM-dd' format
    /// - a number prefixed with `.` is considered a priority
    fn from(item: &str) -> Self {
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
                    t.due = match *token {
                        "@tomorrow" => Utc::now().date().succ().format("%Y-%m-%d").to_string(),
                        "@today" => Utc::now().format("%Y-%m-%d").to_string(),
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
        if t.title.is_empty() {
            panic!("Title of the task is missing")
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
            write!(f, " .{}", self.priority)?;
        }
        if self.finished {
            write!(f, " FINISHED")?;
        }
        write!(f, "")
    }
}
