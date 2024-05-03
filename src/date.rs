use std::cmp::Ordering;
use std::fmt;
use chrono::{Local, Datelike};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Date {
    year: u32,
    month: u32,
    day: u32
}

fn is_leap(year: u32) -> bool {
    if year % 4 != 0 {
        return false;
    }
    if year % 100 != 0 {
        return true;
    }
    return year % 400 == 0;
}

impl Date {
    pub fn new(year: u32, month: u32, day: u32) -> Result<Date, String> {
        if year < 1 {
            return Err("Invalid year. Must be a positive integer.".to_string());
        } else if month < 1 || month > 12 {
            return Err("Invalid month. Must be greater than 0 and less than 12.".to_string());
        }

        let month_days = [31, if is_leap(year) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if day < 1 || day > month_days[month as usize - 1] {
            return Err("Invalid day. Must be greater than 0 and less than {month_days[month as usize-1] - 1}".to_string());
        }
        Ok(Date { year, month, day })
    }

    pub fn now() -> Date {
        let current = Local::now();

        let custom = Date::new(current.year() as u32, current.month(), current.day()).unwrap();
        
        return custom;
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
}

impl Eq for Date {}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.year != other.year {
            self.year.cmp(&other.year)
        } else if self.month != other.month {
            self.month.cmp(&other.month)
        } else {
            self.day.cmp(&other.day)
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Array of month names
        let month_names = [
            "January", "February", "March", "April", "May", "June", "July", "August", "September",
            "October", "November", "December",
        ];
        // Subtract 1 from month to get the index in the month_names array
        let month_name = month_names[self.month as usize - 1];

        write!(f, "{} {}, {}", month_name, self.day, self.year)
    }
}
