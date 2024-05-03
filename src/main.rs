mod date;
mod entry;
mod status;

use date::Date;
use entry::*;
use status::Status;
// use std::io;

fn main() {
    let my_entry = Entry::new(
        "Discussion Board".to_string(),
        "last discussion board post".to_string(),
        // Date::new(2024, 5, 5).unwrap(),
        Date::now(),
        "PH300".to_string(),
        Status::Done,
        1,
        0.0,
    );

    let one_week = my_entry.get_due_date().add_days(7);
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

    let two_weeks = my_entry.get_due_date().add_days(14);
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

    let four_weeks = my_entry.get_due_date().add_days(28);
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

    let one_month = my_entry.get_due_date().add_days(30);
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

    // print all entries
    let mut entries = vec![my_entry, entry4, entry1, entry2, entry3, entry_past];
    for entry in entries.iter() {
        println!("{}", entry);
    }

    // sort by due date
    sort_by_due_date(&mut entries);
    println!("\nSorted by due date:");
    for entry in entries.iter() {
        println!("{}", entry);
    }

    // sort by name
    sort_by_name(&mut entries);
    println!("\nSorted by name:");
    for entry in entries.iter() {
        println!("{}", entry);
    }

    // sort by status
    sort_by_status(&mut entries);
    println!("\nSorted by status:");
    for entry in entries.iter() {
        println!("{}", entry);
    }

    // sort by category
    sort_by_category(&mut entries);
    println!("\nSorted by category:");
    for entry in entries.iter() {
        println!("{}", entry);
    }
}
