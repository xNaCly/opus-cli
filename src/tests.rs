#[cfg(test)]
mod cli {
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
}
