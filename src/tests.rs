#[cfg(test)]
mod cli {
    // #[test]
    // fn parse_export() {
    //     let inputs = [
    //         vec![
    //             "opus".to_owned(),
    //             "export".to_owned(),
    //             "csv".to_owned(),
    //             "tmp1".to_owned(),
    //         ],
    //         vec![
    //             "opus".to_owned(),
    //             "export".to_owned(),
    //             "tsv".to_owned(),
    //             "tmp2".to_owned(),
    //         ],
    //         vec![
    //             "opus".to_owned(),
    //             "export".to_owned(),
    //             "json".to_owned(),
    //             "tmp3".to_owned(),
    //         ],
    //     ];

    //     let expected = [
    //         ArgumentType::Export {
    //             export_type: crate::types::ExportType::Csv,
    //             file_name: "tmp1".to_owned(),
    //         },
    //         ArgumentType::Export {
    //             export_type: crate::types::ExportType::Tsv,
    //             file_name: "tmp2".to_owned(),
    //         },
    //         ArgumentType::Export {
    //             export_type: crate::types::ExportType::Json,
    //             file_name: "tmp3".to_owned(),
    //         },
    //     ];

    //     for (input, expected) in inputs.iter().zip(expected) {
    //         assert_eq!(parse_args(input.to_owned()).top_level_arg, expected)
    //     }
    // }

    // #[test]
    // #[should_panic]
    // fn not_enough_arguments_ii() {
    //     parse_args(vec!["opus".to_string(), "add".to_string()]);
    // }

    // #[test]
    // #[should_panic]
    // fn not_enough_arguments_export_1() {
    //     parse_args(vec!["opus".to_string(), "export".to_string()]);
    // }

    // #[test]
    // #[should_panic]
    // fn not_enough_arguments_export_2() {
    //     parse_args(vec![
    //         "opus".to_string(),
    //         "export".to_string(),
    //         "csv".to_owned(),
    //     ]);
    // }

    // #[test]
    // #[should_panic]
    // fn not_enough_arguments_export_3() {
    //     parse_args(vec![
    //         "opus".to_string(),
    //         "export".to_string(),
    //         "file".to_owned(),
    //     ]);
    // }

    use crate::cli::{cli_clear, cli_del_task, cli_fin_task, cli_get_tasks};

    #[test]
    fn insert_task() {
        use crate::cli::cli_add_task;
        use crate::{db::open_db, types::Task};

        let task = Task::from(String::from("update excel sheet #work @today .5"));
        let db = open_db();

        db.create_table_if_missing();
        cli_add_task(&db, task);

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn get_task_by_id() {
        use crate::cli::cli_add_task;
        use crate::{db::open_db, types::Task};

        let task = Task::from(String::from("update excel sheet #work @today .5"));
        let db = open_db();

        db.create_table_if_missing();
        cli_add_task(&db, task.clone());

        let last_id = db.con.last_insert_rowid();

        let tasks = cli_get_tasks(&db, last_id.to_string(), false);
        let task1 = tasks.get(0).unwrap();

        assert_eq!(task.title, task1.title);
        assert_eq!(task.tag, task1.tag);
        assert_eq!(task.priority, task1.priority);
        assert_eq!(task.due, task1.due);
        assert_eq!(task.finished, task1.finished);

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn get_task_by_tag() {
        use crate::cli::cli_add_task;
        use crate::{db::open_db, types::Task};

        let task = Task::from(String::from("update excel sheet #test @today .5"));
        let db = open_db();

        db.create_table_if_missing();
        cli_add_task(&db, task.clone());

        let tasks = cli_get_tasks(&db, "#test".to_string(), false);
        let task1 = tasks.get(0).unwrap();

        assert_eq!(task.title, task1.title);
        assert_eq!(task.tag, task1.tag);
        assert_eq!(task.priority, task1.priority);
        assert_eq!(task.due, task1.due);
        assert_eq!(task.finished, task1.finished);

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn get_task_by_prio() {
        use crate::cli::cli_add_task;
        use crate::{db::open_db, types::Task};

        let task = Task::from(String::from("update excel sheet #work @today .18"));
        let db = open_db();

        db.create_table_if_missing();
        cli_add_task(&db, task.clone());

        let tasks = cli_get_tasks(&db, ".18".to_string(), false);
        let task1 = tasks.get(0).unwrap();

        assert_eq!(task.title, task1.title);
        assert_eq!(task.tag, task1.tag);
        assert_eq!(task.priority, task1.priority);
        assert_eq!(task.due, task1.due);
        assert_eq!(task.finished, task1.finished);

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn finish_task() {
        use crate::cli::cli_add_task;
        use crate::{db::open_db, types::Task};

        let task = Task::from(String::from("update excel sheet #work @today .5"));
        let db = open_db();

        db.create_table_if_missing();
        cli_add_task(&db, task.clone());

        let id = db.con.last_insert_rowid();

        cli_fin_task(&db, id.clone().to_string());

        let tasks = cli_get_tasks(&db, id.to_string(), true);
        let task1 = tasks.get(0).unwrap();

        assert!(task1.finished);

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn clear_tasks() {
        use crate::cli::cli_add_task;
        use crate::{db::open_db, types::Task};

        let task = Task::from(String::from("update excel sheet #work @today .5"));
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

        let task = Task::from(String::from("update excel sheet #delete @today .5"));
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
        let task_string = String::from("i got stuff to do #work .5 @2022-10-17");
        let t: Task = Task::from(task_string);
        assert!(!t.finished);
        assert_eq!(t.priority, 5);
        assert_eq!(t.due, String::from("2022-10-17"));
        assert_eq!(t.title, String::from("i got stuff to do"));
        assert_eq!(t.tag, String::from("#work"));
    }
}
