//! # Opus database wrapper
use rusqlite::Connection;

use crate::{types::{Task, ExportType}, util::create_dir_if_not_exist, util::get_db_path};

pub struct Database {
    pub con: Connection,
}

pub const CREATE_TABLE_IF_MISSING: &str = "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY, title TEXT, tag TEXT, due TEXT, priority INTEGER, finished INTEGER)";
pub const GET_TASK_BY_ID: &str = "SELECT * FROM tasks WHERE id IS ?";
pub const GET_ALL_TASKS: &str = "SELECT * FROM tasks";
pub const FINISH_TASK_BY_ID: &str = "UPDATE tasks SET finished = 1 WHERE id IS ?";
pub const GET_TASK_BY_TAG: &str = "SELECT * FROM tasks WHERE tag IS ?";
pub const GET_TASK_BY_PRIO: &str = "SELECT * FROM tasks WHERE priority IS ?";
pub const REMOVE_ALL_TASKS: &str = "DELETE FROM tasks";
pub const INSERT_TASK: &str =
    "INSERT INTO tasks (title, tag, due, priority, finished) VALUES(?,?,?,?,?)";

pub fn open_db() -> Database {
    let path = get_db_path();
    create_dir_if_not_exist(&path);
    match Connection::open(path) {
        Ok(con) => Database { con },
        Err(_) => panic!("Couldn't open database!"),
    }
}

impl Database {
    /// gets a vector of tasks matching the given property and query
    ///
    /// property:
    ///  - `#`: task tag
    ///  - `,`: task prio
    ///  - `@`: task due date
    ///
    /// Property is generally the first char of the query. Matching the property type is required to choose the correct database query.
    ///
    /// Caviats:
    /// - this method only returns open tasks (not finished)
    pub fn get_tasks(&self, property: char, mut query: String) -> Vec<Task> {
        let mut sql_query = match property {
            '#' => GET_TASK_BY_TAG,
            ',' => GET_TASK_BY_PRIO,
            '@' => {
                unimplemented!("querying via date will be implemented in the future");
            }
            _ => GET_TASK_BY_ID,
        };

        if query == "ls" || query == "l" {
            sql_query = GET_ALL_TASKS;
        }

        let mut stmt = self
            .con
            .prepare(sql_query)
            .expect("Failed to prepare SQL statement in querying for tasks");

        if query == "ls" || query == "l" {
            return stmt
                .query_map([], |row| {
                    Ok(Task {
                        id: row.get("id")?,
                        title: row.get("title")?,
                        tag: row.get("tag")?,
                        due: row.get("due")?,
                        priority: row.get("priority")?,
                        finished: matches!(row.get("finished")?, 1),
                    })
                })
                .expect("Failed to query all tasks")
                .map(|x| x.expect("Couldn't map over tasks returned by database"))
                // TODO: remove this if queried by id
                .filter(|x| !x.finished)
                .collect::<Vec<Task>>();
        }

        if sql_query == GET_TASK_BY_PRIO {
            query = query.len().to_string();
        }

        stmt.query_map([query], |row| {
            Ok(Task {
                id: row.get("id")?,
                title: row.get("title")?,
                tag: row.get("tag")?,
                due: row.get("due")?,
                priority: row.get("priority")?,
                finished: matches!(row.get("finished")?, 1),
            })
        })
        .expect("Couldn't get task with the given query")
        .map(|x| x.expect("Couldn't map over tasks returned by database"))
        // TODO: remove
        .filter(|x| !x.finished)
        .collect::<Vec<Task>>()
    }

    pub fn create_table_if_missing(&self) {
        self.con
            .execute(CREATE_TABLE_IF_MISSING, [])
            .expect("Creating task table in database failed");
    }

    pub fn insert_task(&self, t: Task) {
        let finished = if t.finished { 1 } else { 0 };
        self.con
            .execute(
                INSERT_TASK,
                [
                    t.title,
                    t.tag,
                    t.due,
                    t.priority.to_string(),
                    finished.to_string(),
                ],
            )
            .expect("Couldn't insert task into database");
    }

    /// marks a task as finished in the database
    ///
    /// returns: amount of rows affected
    pub fn mark_task_as_finished(&self, id: usize) -> usize {
        self.con
            .execute(FINISH_TASK_BY_ID, [id.to_string()])
            .expect("Couldn't finish task")
    }

    pub fn clear_all_tasks(&self) -> usize {
        self.con
            .execute(REMOVE_ALL_TASKS, [])
            .expect("Clearing database failed")
    }

    pub fn export(&self, export_type: &ExportType) -> String {
        let tasks = self.get_tasks('l', "l".to_string());

        let tasks = match export_type {
            ExportType::Json => serde_json::to_string(&tasks).map_err(|_| "Exporting failed".to_owned()),
            _ => {
                let sep = match export_type{
                    ExportType::Csv => ",",
                    _ => "\t",
                };

                Ok(tasks.iter()
                .map(|t| format!("{}{sep}{}{sep}{}{sep}{}{sep}{}\n", t.title, t.tag, t.priority, t.due, t.finished))
                .collect::<String>())
            }
        };

        tasks.expect("Exporting failed")
    }
}
