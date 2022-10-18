#[cfg(test)]
use crate::types::Task;
#[cfg(test)]
impl Task {
    fn content_compare(&self, t: &Task) -> bool {
        let titles = self.title == t.title;
        let tag = self.tag == t.tag;
        let prio = self.priority == t.priority;
        let due = self.due == t.due;
        let finished = self.finished == t.finished;
        titles && tag && prio && due && finished
    }
}

#[cfg(test)]
mod cli {
    use crate::cli::{cli_clear, cli_del_task, cli_fin_task, cli_get_tasks};

    #[test]
    fn insert_task() {
        use crate::cli::cli_add_task;
        use crate::{db::open_db, types::Task};

        let task = Task::from("update excel sheet #work @today .5");
        let db = open_db();

        db.create_table_if_missing();
        cli_add_task(&db, task);

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn get_task_by_id() {
        use crate::cli::cli_add_task;
        use crate::{db::open_db, types::Task};

        let task = Task::from("update excel sheet #work @today .5");
        let db = open_db();

        db.create_table_if_missing();
        cli_add_task(&db, task.clone());

        let last_id = db.con.last_insert_rowid();

        let tasks = cli_get_tasks(&db, last_id.to_string(), false);
        let task1 = tasks.get(0).unwrap();

        assert!(task.content_compare(task1));

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn get_task_by_tag() {
        use crate::cli::cli_add_task;
        use crate::{db::open_db, types::Task};

        let task = Task::from("update excel sheet #test @today .5");
        let db = open_db();

        db.create_table_if_missing();
        cli_add_task(&db, task.clone());

        let tasks = cli_get_tasks(&db, "#test".to_string(), false);
        let task1 = tasks.get(0).unwrap();

        assert!(task.content_compare(task1));

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn get_task_by_prio() {
        use crate::cli::cli_add_task;
        use crate::{db::open_db, types::Task};

        let task = Task::from("update excel sheet #work @today .18");
        let db = open_db();

        db.create_table_if_missing();
        cli_add_task(&db, task.clone());

        let tasks = cli_get_tasks(&db, ".18".to_string(), false);
        let task1 = tasks.get(0).unwrap();

        assert!(task.content_compare(task1));

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn finish_task() {
        use crate::cli::cli_add_task;
        use crate::{db::open_db, types::Task};

        let mut task = Task::from("update excel sheet #work @today .5");
        let db = open_db();

        db.create_table_if_missing();
        cli_add_task(&db, task.clone());

        let id = db.con.last_insert_rowid();

        cli_fin_task(&db, id.to_string());

        let tasks = cli_get_tasks(&db, id.to_string(), true);
        let task1 = tasks.get(0).unwrap();

        task.finished = true;
        assert!(task.content_compare(task1));

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn clear_tasks() {
        use crate::cli::cli_add_task;
        use crate::{db::open_db, types::Task};

        let task = Task::from("update excel sheet #work @today .5");
        let db = open_db();

        db.create_table_if_missing();
        cli_add_task(&db, task.clone());
        cli_clear(&db);

        let tasks = cli_get_tasks(&db, "list".to_string(), true);

        assert_eq!(tasks.len(), 0);

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn delete_task() {
        use crate::cli::cli_add_task;
        use crate::{db::open_db, types::Task};

        let task = Task::from("update excel sheet #delete @today .5");
        let db = open_db();

        db.create_table_if_missing();
        cli_add_task(&db, task.clone());

        let id = db.con.last_insert_rowid();

        cli_del_task(&db, id.to_string());

        let tasks = cli_get_tasks(&db, "#delete".to_string(), true);

        assert_eq!(tasks.len(), 0);

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn export_tasks() {
        use crate::{db::open_db, types::Task};

        let db = open_db();
        db.create_table_if_missing();

        let output = db.export(&crate::types::ExportType::Csv);
        assert_eq!(output, "");

        db.insert_task(Task {
            id: Some(1_usize),
            title: "title".to_owned(),
            tag: "tag".to_owned(),
            priority: 2_usize,
            due: "due".to_owned(),
            finished: false,
        });

        let output = db.export(&crate::types::ExportType::Csv);
        assert_eq!(output, "title,tag,2,due,false\n");
    }
}

#[cfg(test)]
mod util {
    use std::path::Path;

    use crate::util::{create_dir_if_not_exist, get_db_path};

    #[test]
    fn get_database_path() {
        assert!(!(get_db_path().is_empty()));
    }

    #[test]
    fn create_dir_if_no_exist() {
        let path = get_db_path();
        assert!(create_dir_if_not_exist(&path));
        let ppath = Path::new(&path);
        assert!(ppath
            .parent()
            .expect("Couldn't get directory of db file")
            .exists());
    }
}

#[cfg(test)]
mod db {
    use crate::db::open_db;

    #[test]
    fn open_connection() {
        let db = open_db();
        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn create_default_table() {
        let db = open_db();
        db.create_table_if_missing();
        db.con.close().expect("Closing Database failed.");
    }
}

mod types {

    #[test]
    fn parse_task() {
        use crate::types::Task;
        let task_string = "i got stuff to do #work .5 @2022-10-17";
        let t: Task = Task::from(task_string);
        assert!(!t.finished);
        assert_eq!(t.priority, 5);
        assert_eq!(t.due, String::from("2022-10-17"));
        assert_eq!(t.title, String::from("i got stuff to do"));
        assert_eq!(t.tag, String::from("#work"));
    }
}
