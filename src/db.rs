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
use rusqlite::{Connection, Result};

use crate::util::get_db_path;

pub fn new_db_connection() {
    let path = get_db_path();
    unimplemented!();
}
