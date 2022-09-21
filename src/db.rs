//! # Opus database wrapper
use rusqlite::Connection;

use crate::{
    types::{Database, Task},
    util::get_db_path,
};

pub const CREATE_TABLE_IF_MISSING: &str = "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY, title TEXT, tag TEXT, due TEXT, priority TEXT)";
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
    pub fn get_task_by_id(&self, id: usize) -> Task {
        unimplemented!()
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
