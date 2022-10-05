#[cfg(test)]
mod cli {
    use crate::{
        cli::{cli_add_task, cli_fin_task, cli_get_tasks, parse_args},
        db::open_db,
        types::ArgumentType,
    };
    #[test]
    fn parse_arguments_add() {
        let r = parse_args(vec![
            "opus".to_string(),
            "add".to_string(),
            "update excel sheet #work @today ,,,".to_string(),
        ]);
        let task = r.input.task.unwrap();
        assert_eq!(task.title, "update excel sheet");
        assert_eq!(task.tag, "#work");
        assert_eq!(task.due, "@today");
        assert_eq!(task.priority, 3);
        assert!(!task.finished);
    }

    #[test]
    fn parse_arguments_args_type() {
        let args = ["add", "del", "clear", "fin", "ls"];
        let args_types = [
            ArgumentType::Add,
            ArgumentType::Delete,
            ArgumentType::Clear,
            ArgumentType::Finish,
            ArgumentType::List,
        ];
        for i in 0..4 {
            let r = parse_args(vec![
                "opus".to_string(),
                args[i].to_string(),
                "arg2".to_string(),
            ]);
            assert_eq!(r.top_level_arg, args_types[i]);
        }
    }

    #[test]
    #[should_panic]
    fn not_enough_arguments() {
        parse_args(vec!["opus".to_string()]);
    }
    #[test]
    #[should_panic]
    fn not_enough_arguments_ii() {
        parse_args(vec!["opus".to_string(), "add".to_string()]);
    }
    #[test]
    fn insert_task() {
        let r = parse_args(vec![
            "opus".to_string(),
            "add".to_string(),
            "update excel sheet #work @today ,,,".to_string(),
        ]);
        let task = r.input.task.unwrap();
        let db = open_db();
        db.create_table_if_missing();
        cli_add_task(&db, task);
        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn get_task_by_id() {
        let r = parse_args(vec![
            "opus".to_string(),
            "add".to_string(),
            "update excel sheet #work @today ,,,".to_string(),
        ]);
        let task = r.input.task.unwrap();
        let db = open_db();
        db.create_table_if_missing();

        cli_add_task(&db, task);
        let tasks = cli_get_tasks(&db, db.con.last_insert_rowid().to_string());
        let task = tasks.get(0).unwrap();

        assert_eq!(task.title, "update excel sheet");
        assert_eq!(task.tag, "#work");
        assert_eq!(task.priority, 3);

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn get_task_by_tag() {
        let r = parse_args(vec![
            "opus".to_string(),
            "add".to_string(),
            "update excel sheet #work @today ,,,".to_string(),
        ]);
        let task = r.input.task.unwrap();

        let db = open_db();
        db.create_table_if_missing();

        cli_add_task(&db, task);
        let tasks = cli_get_tasks(&db, "#work".to_string());
        let task = tasks.get(0).unwrap();

        assert_eq!(task.title, "update excel sheet");
        assert_eq!(task.tag, "#work");
        assert_eq!(task.priority, 3);

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn get_task_by_prio() {
        let r = parse_args(vec![
            "opus".to_string(),
            "add".to_string(),
            "update excel sheet #work @today ,,,".to_string(),
        ]);
        let task = r.input.task.unwrap();

        let db = open_db();
        db.create_table_if_missing();

        cli_add_task(&db, task);
        let tasks = cli_get_tasks(&db, ",,,".to_string());
        let task = tasks.get(0).unwrap();

        assert_eq!(task.title, "update excel sheet");
        assert_eq!(task.tag, "#work");
        assert_eq!(task.priority, 3);

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn finish_task() {
        let r = parse_args(vec![
            "opus".to_string(),
            "add".to_string(),
            "update excel sheet #work @today ,,,".to_string(),
        ]);
        let task = r.input.task.unwrap();
        let db = open_db();

        db.create_table_if_missing();

        cli_add_task(&db, task);
        let id = db.con.last_insert_rowid().to_string();

        cli_fin_task(&db, id.clone());

        let tasks = db.get_tasks('0', id);
        assert_eq!(tasks.len(), 0);

        db.con.close().expect("Closing Database failed.");
    }

    #[test]
    fn clear_tasks() {
        let db = open_db();

        db.create_table_if_missing();
        db.clear_all_tasks();
        let tasks = db.get_tasks('l', "ls".to_string()).len();
        assert_eq!(tasks, 0);
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
        create_dir_if_not_exist(&path);
        let ppath = Path::new(&path);
        assert!(ppath.exists());
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
