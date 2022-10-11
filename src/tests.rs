#[cfg(test)]
mod cli {
    use std::vec;

    use crate::{
        cli::{cli_add_task, cli_fin_task, cli_get_tasks, parse_args},
        db::open_db,
        types::{ArgumentType, Task},
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
        let args = ["add", "delete", "clear", "finish", "list", "export"];
        let args1 = ["a", "d", "clear", "f", "l", "export"];
        let args_types = [
            ArgumentType::Add,
            ArgumentType::Delete,
            ArgumentType::Clear,
            ArgumentType::Finish,
            ArgumentType::List,
        ];
        for (arg, arg_type) in args.iter().zip(args_types.clone()) {
            let r = parse_args(vec![
                "opus".to_string(),
                arg.to_string(),
                "arg2".to_string(),
            ]);
            assert_eq!(r.top_level_arg, arg_type);
        }
        for (arg1, arg_type) in args.iter().zip(args_types) {
            let r = parse_args(vec![
                "opus".to_string(),
                arg1.to_string(),
                "arg2".to_string(),
            ]);
            assert_eq!(r.top_level_arg, arg_type);
        }
    }

    #[test]
    fn parse_export() {
        let inputs = [
            vec![
                "opus".to_owned(),
                "export".to_owned(),
                "csv".to_owned(),
                "tmp1".to_owned(),
            ],
            vec![
                "opus".to_owned(),
                "export".to_owned(),
                "tsv".to_owned(),
                "tmp2".to_owned(),
            ],
            vec![
                "opus".to_owned(),
                "export".to_owned(),
                "json".to_owned(),
                "tmp3".to_owned(),
            ],
        ];

        let expected = [
            ArgumentType::Export {
                export_type: crate::types::ExportType::Csv,
                file_name: "tmp1".to_owned(),
            },
            ArgumentType::Export {
                export_type: crate::types::ExportType::Tsv,
                file_name: "tmp2".to_owned(),
            },
            ArgumentType::Export {
                export_type: crate::types::ExportType::Json,
                file_name: "tmp3".to_owned(),
            },
        ];

        for (input, expected) in inputs.iter().zip(expected) {
            assert_eq!(parse_args(input.to_owned()).top_level_arg, expected)
        }
    }

    #[test]
    #[should_panic]
    fn not_enough_arguments_ii() {
        parse_args(vec!["opus".to_string(), "add".to_string()]);
    }

    #[test]
    #[should_panic]
    fn not_enough_arguments_export_1() {
        parse_args(vec!["opus".to_string(), "export".to_string()]);
    }

    #[test]
    #[should_panic]
    fn not_enough_arguments_export_2() {
        parse_args(vec![
            "opus".to_string(),
            "export".to_string(),
            "csv".to_owned(),
        ]);
    }

    #[test]
    #[should_panic]
    fn not_enough_arguments_export_3() {
        parse_args(vec![
            "opus".to_string(),
            "export".to_string(),
            "file".to_owned(),
        ]);
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
        let r = parse_args(vec![
            "opus".to_string(),
            "add".to_string(),
            "update excel sheet #work @today ,,,".to_string(),
        ]);
        let task = r.input.task.unwrap();
        let db = open_db();

        db.create_table_if_missing();
        db.clear_all_tasks();

        let tasks = db.get_tasks('l', "l".to_string()).len();
        assert_eq!(tasks, 0);
    }

    #[test]
    fn delete_task() {
        let r = parse_args(vec![
            "opus".to_string(),
            "add".to_string(),
            "should be deleted #delete @today ,,,".to_string(),
        ]);
        let task = r.input.task.unwrap();
        let db = open_db();

        db.create_table_if_missing();

        cli_add_task(&db, task);
        let id = db.con.last_insert_rowid().to_string();
        db.delete_task(id.parse::<usize>().unwrap());

        let tasks = cli_get_tasks(&db, "#delete".to_string()).len();
        assert_eq!(tasks, 0);
    }

    #[test]
    fn export_tasks() {
        let db = open_db();

        db.create_table_if_missing();
        db.clear_all_tasks();
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
