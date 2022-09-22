//! # Opus database wrapper
use rusqlite::Connection;

use crate::{
    types::{Database, Task},
    util::get_db_path,
};

pub const CREATE_TABLE_IF_MISSING: &str = "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY, title TEXT, tag TEXT, due TEXT, priority INTEGER)";
pub const GET_TASK_BY_ID: &str = "SELECT * FROM tasks WHERE id IS ?";
pub const GET_TASK_BY_TAG: &str = "SELECT * FROM tasks WHERE tag IS ?";
pub const GET_TASK_BY_PRIO: &str = "SELECT * FROM tasks WHERE priority IS ?";
pub const INSERT_TASK: &str = "INSERT INTO tasks (title, tag, due, priority) VALUES(?,?,?,?)";

pub fn open_db() -> Database {
    let path = get_db_path();

    dbg!(&path);

    match Connection::open(path) {
        Ok(con) => Database { con },
        Err(_) => panic!("Couldn't open database!"),
    }
}

impl Database {
    pub fn get_tasks(&self, property: char, mut query: String) -> Vec<Task> {
        let sql_query = match property {
            '#' => GET_TASK_BY_TAG,
            ',' => GET_TASK_BY_PRIO,
            '@' => {
                unimplemented!("querying via date will be implemented in the future");
            }
            _ => GET_TASK_BY_ID,
        };

        let mut stmt = self
            .con
            .prepare(sql_query)
            .expect("Failed to prepare sql statment in querying for tasks");

        if sql_query == GET_TASK_BY_PRIO {
            query = query.len().to_string();
        }

        stmt.query_map([query], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                tag: row.get(2)?,
                due: row.get(3)?,
                priority: row.get(4)?,
            })
        })
        .expect("Couldn't get task with the given id")
        .map(|x| x.expect("Couldn't map over tasks returned by database"))
        .collect::<Vec<Task>>()
    }

    pub fn create_table_if_missing(&self) {
        self.con
            .execute(CREATE_TABLE_IF_MISSING, [])
            .expect("Creating task table in database failed");
    }

    pub fn insert_task(&self, t: Task) {
        self.con
            .execute(INSERT_TASK, [t.title, t.tag, t.due, t.priority.to_string()])
            .expect("Couldn't insert task into database");
    }
}
