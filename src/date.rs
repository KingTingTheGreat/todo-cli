use crate::colors::{DUE_FOUR_WEEKS, DUE_ONE_WEEK, DUE_THREE_DAYS, DUE_TWO_WEEKS, PAST_DUE_COLOR};
use chrono::{Datelike, Local};
use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

fn is_leap(year: u16) -> bool {
    if year % 4 != 0 {
        return false;
    }
    if year % 100 != 0 {
        return true;
    }
    return year % 400 == 0;
}

fn get_num_days(year: u16, month: u8) -> u8 {
    let month_days: [u8; 12] = [
        31,
        if is_leap(year) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    return month_days[month as usize - 1];
}

fn get_month_name(month: u8) -> String {
    let month_names = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    return month_names[month as usize - 1].to_string();
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Result<Date, String> {
        if year < 1 {
            return Err("Invalid year. Must be a positive integer.".to_string());
        } else if month < 1 || month > 12 {
            return Err("Invalid month. Must be greater than 0 and less than 12.".to_string());
        }

        let max_days = get_num_days(year, month);
        if day < 1 || day > max_days {
            return Err("Invalid day. Must be greater than 0 and less than {max_days}".to_string());
        }
        Ok(Date { year, month, day })
    }

    pub fn now() -> Date {
        let current = Local::now();

        let custom = Date::new(
            current.year() as u16,
            current.month() as u8,
            current.day() as u8,
        )
        .unwrap();

        return custom;
    }

    pub fn get_year(&self) -> u16 {
        self.year
    }

    pub fn get_month(&self) -> u8 {
        self.month
    }

    pub fn get_day(&self) -> u8 {
        self.day
    }

    pub fn set_year(&mut self, year: u16) {
        self.year = year;
    }

    pub fn set_month(&mut self, month: u8) {
        self.month = month;
    }

    pub fn set_day(&mut self, day: u8) {
        self.day = day;
    }

    pub fn add_days(&self, days: u16) -> Date {
        let mut new_date = Date {
            year: self.year,
            month: self.month,
            day: self.day,
        };

        let mut days_left = days;

        while days_left > 0 {
            let diff = get_num_days(new_date.year, new_date.month) - new_date.day;
            if days_left <= diff as u16 {
                new_date.day += days_left as u8;
                days_left = 0;
            } else {
                new_date.day = 1;
                if new_date.month == 12 {
                    new_date.month = 1;
                    new_date.year += 1;
                } else {
                    new_date.month += 1;
                }
                days_left -= diff as u16;
            }
        }

        return new_date;
    }

    pub fn to_colored_string(&self) -> ColoredString {
        let today = Date::now();
        if *self < today {
            return self.to_string().custom_color(PAST_DUE_COLOR);
        } else if *self <= today.add_days(3) {
            return self.to_string().custom_color(DUE_THREE_DAYS);
        } else if *self <= today.add_days(7) {
            return self.to_string().custom_color(DUE_ONE_WEEK);
        } else if *self <= today.add_days(14) {
            return self.to_string().custom_color(DUE_TWO_WEEKS);
        } else if *self <= today.add_days(28) {
            return self.to_string().custom_color(DUE_FOUR_WEEKS);
        } else {
            return self.to_string().normal();
        };
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
        let month_name = get_month_name(self.month);

        write!(f, "{} {}, {}", month_name, self.day, self.year)
    }
}
