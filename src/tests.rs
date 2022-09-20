//! Opus integration and unit tests
#[cfg(test)]
mod cli {
    use std::vec;

    #[test]
    fn parse_args() {
        use crate::cli::parse_args;
        let r = parse_args(vec![
            "opus".to_string(),
            "add".to_string(),
            "update excel sheet #work @today |||".to_string(),
        ]);
        dbg!(&r);
        let task = r.input.task.unwrap();
        assert_eq!(task.title, "update excel sheet");
        assert_eq!(task.tag, "#work");
        assert_eq!(task.due, "@today");
        assert_eq!(task.priority, 3);
    }
    #[test]
    #[should_panic]
    fn not_enough_arguments() {
        use crate::cli::parse_args;
        parse_args(vec!["opus".to_string()]);
    }
    #[test]
    #[should_panic]
    fn not_enough_arguments_ii() {
        use crate::cli::parse_args;
        parse_args(vec!["opus".to_string(), "add".to_string()]);
    }
}

#[cfg(test)]
mod db {
    use crate::db::open_db;

    #[test]
    fn get_db_path() {
        use crate::util::get_db_path;
        dbg!(get_db_path());
    }

    #[test]
    fn open_connection() {
        open_db();
    }
}
