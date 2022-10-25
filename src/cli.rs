use crate::db::Database;
use crate::types::{ExportType, SortMode, SortOrder, Task};

/// add the given Task to the database
pub fn cli_add_task(db: &Database, t: Task) {
    if t.title.is_empty() {
        panic!(
            "Task '{:?}' has no title, a task's title is the only required value!",
            t
        );
    }

    let task = Task {
        id: None,
        title: t.title,
        tag: t.tag,
        priority: t.priority,
        due: t.due,
        finished: false,
    };

    db.insert_task(task);
}

pub fn cli_del_task(db: &Database, id: String) -> bool {
    db.delete_task(id.parse::<usize>().expect("Given id wasn't an integer")) != 0
}

pub fn cli_fin_task(db: &Database, id: String) -> bool {
    db.mark_task_as_finished(id.parse::<usize>().expect("Given id wasn't an integer")) != 0
}

pub fn cli_get_tasks(
    db: &Database,
    q: String,
    display_finished: bool,
    sort_by: SortMode,
    sort_order: SortOrder,
) -> Vec<Task> {
    db.get_tasks(
        q.chars().next().expect("Failure in getting task query"),
        q,
        display_finished,
        sort_by,
        sort_order,
    )
}

pub fn cli_clear(db: &Database) -> bool {
    db.clear_all_tasks() != 0
}

pub fn cli_export(db: &Database, export_type: &ExportType) -> String {
    db.export(export_type)
}

pub fn cli_import(db: &Database, filecontent: Vec<String>) -> bool {
    for l in filecontent {
        println!("{}", l);
    }
    false
}
