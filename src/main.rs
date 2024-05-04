mod colors;
mod date;
mod entry;
mod status;
mod todo_list;

use date::Date;
use entry::*;
use status::Status;
use todo_list::TodoList;

const ENTRIES_FILENAME: &str = "todo-entries.json";

fn init_saved() -> TodoList {
    let todo_list = TodoList::from_file(ENTRIES_FILENAME);
    return todo_list;
}

fn persist(todo_list: &TodoList) {
    todo_list.to_file(ENTRIES_FILENAME);
}

fn main() {
    let today = Date::now();

    let one_week = today.add_days(7);
    let entry1 = Entry::new(
        "Final Paper".to_string(),
        "final paper".to_string(),
        // Date::new(2024, 5, 5).unwrap(),
        one_week,
        "PH300".to_string(),
        Status::InProgress,
        1,
        0.0,
    );

    let two_weeks = today.add_days(14);
    let entry2 = Entry::new(
        "Final Paper".to_string(),
        "last discussion board post".to_string(),
        // Date::new(2024, 5, 5).unwrap(),
        two_weeks,
        "WS335".to_string(),
        Status::InProgress,
        1,
        0.0,
    );

    let four_weeks = today.add_days(28);
    let entry3 = Entry::new(
        "Final Exam".to_string(),
        "last discussion board post".to_string(),
        // Date::new(2024, 5, 5).unwrap(),
        four_weeks,
        "CS350".to_string(),
        Status::InProgress,
        1,
        0.0,
    );

    let one_month = today.add_days(30);
    let entry4 = Entry::new(
        "Onboarding".to_string(),
        "last discussion board ".to_string(),
        // Date::new(2024, 5, 5).unwrap(),
        one_month,
        "Cargill".to_string(),
        Status::NotStarted,
        1,
        0.0,
    );

    let entry_past = Entry::new(
        "OSN".to_string(),
        "last discussion board post".to_string(),
        // Date::new(2024, 5, 5).unwrap(),
        Date::new(2020, 5, 5).unwrap(),
        "Cargill".to_string(),
        Status::NotStarted,
        1,
        0.0,
    );

    let mut todo_list = init_saved();

    todo_list.clear();

    todo_list.add_entries(vec![entry1, entry2, entry3, entry4, entry_past]);

    // print all entries
    todo_list.print_entries(&[NAME, CATEGORY, DUE_DATE, STATUS]);

    // sort by due date
    todo_list.sort_by_due_date();
    println!("\nSorted by due date:");
    todo_list.print_entries(&[NAME, CATEGORY, DUE_DATE, STATUS]);

    persist(&todo_list);
}
