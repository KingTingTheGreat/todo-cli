mod date;
mod entry;
mod status;

use date::Date;
use entry::Entry;
use status::Status;
// use colored::*;
// use std::io;

fn main() {
    let date1 = Date::new(2024, 5, 2).unwrap();
    let date2 = Date::new(2024, 5, 3).unwrap();
    let date3 = Date::new(2023, 12, 31).unwrap();
    println!("Date 1 == Date 2: {}", date1 == date2); // true
    println!("Date 1 != Date 3: {}", date1 != date3); // true

    println!("Date 1 < Date 3: {}", date1 < date3); // false
    println!("Date 1 > Date 3: {}", date1 > date3); // true
    let _entry = Entry::new(
        "my entry".to_string(),
        "my desc".to_string(),
        date3,
        "my category".to_string(),
        Status::Done,
        0,
        0.0,
    );

    println!("{}", date1);

    let mut dates = vec![
        Date::new(2024, 5, 2).unwrap(),
        Date::new(2023, 12, 31).unwrap(),
        Date::new(2025, 1, 15).unwrap(),
        Date::new(2024, 10, 8).unwrap(),
    ];

    println!("Before sorting: {:?}", dates);

    dates.sort(); // Sort the dates

    println!("After sorting: {:?}", dates);

    // Trying to create an invalid date
    let invalid_date = Date::new(2024, 13, 32);
    match invalid_date {
        Ok(_) => println!("Invalid date: This should not be reached"),
        Err(err) => println!("Error: {}", err), // Prints: "Error: Invalid date"
    }

    let custom_now = Date::now();
    println!("custom now: {}", custom_now);

    let mut entries = vec![
        Entry::new(
            "entry 1".to_string(),
            "desc 1".to_string(),
            Date::new(2023, 12, 31).unwrap(),
            "category 1".to_string(),
            Status::Done,
            0,
            0.0,
        ),
        Entry::new(
            "entry 2".to_string(),
            "desc 2".to_string(),
            Date::new(2024, 10, 8).unwrap(),
            "category 2".to_string(),
            Status::Pending,
            1,
            1.0,
        ),
        Entry::new(
            "entry 3".to_string(),
            "desc 3".to_string(),
            Date::new(2025, 1, 15).unwrap(),
            "category 3".to_string(),
            Status::NotStarted,
            2,
            2.0,
        ),
    ];

    entry::sort(&mut entries, entry::SortType::Name);
    println!("Entries sorted by name: {:#?}", entries);

    entry::sort(&mut entries, entry::SortType::Status);
    println!("Entries sorted by due date: {:#?}", entries);
}
