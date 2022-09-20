// todo: Thinking about the implementation of a database:
//! # Thinking about the implementation of a database:
//!
//! maybe switch from a file based approch to a sqlite database, the currently planned implementation has the following flaws, which could be removed by using a relational database instead of a file:
//!
//! - deleting a row would cause the program to read the whole file, remove the row and write the whole file minus the row back to disk (this is a waste of ressources). Using sql one can just `DELETE` an entry
//!
//! - to keep tabs on the current id, the program needs to read all lines from the disk and increment the last id. Sqlite autoincrements the primary id.
//!
//! - `opus list` requires a query engine, which is already included in sql, but not in this program
//!
//!
//! # Opus database wrapper
use rusqlite::Connection;

use crate::{types::Task, util::get_db_path};

pub fn open_db() -> Connection {
    let path = get_db_path();

    dbg!(&path);

    match Connection::open(path) {
        Ok(con) => con,
        Err(_) => panic!("Couldn't open database!"),
    }
}

pub fn get_task_by_id(id: usize) -> Task {
    unimplemented!()
}
