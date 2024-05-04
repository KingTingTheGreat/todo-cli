use crate::Date;
use crate::Status;
use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use std::fmt;

pub const NAME: &str = "name";
pub const DESCRIPTION: &str = "description";
pub const DATE_ADDED: &str = "date_added";
pub const DUE_DATE: &str = "due_date";
pub const CATEGORY: &str = "category";
pub const STATUS: &str = "status";
pub const PRIORITY: &str = "priority";
pub const COST: &str = "cost";

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    name: String,
    description: String,
    date_added: Date,
    due_date: Date,
    category: String,
    status: Status,
    priority: i32,
    cost: f32,
}

impl Entry {
    pub fn new(
        name: String,
        description: String,
        due_date: Date,
        category: String,
        status: Status,
        priority: i32,
        cost: f32,
    ) -> Entry {
        Entry {
            name,
            description,
            date_added: Date::now(),
            due_date,
            category,
            status,
            priority,
            cost,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_date_added(&self) -> &Date {
        &self.date_added
    }

    pub fn get_due_date(&self) -> &Date {
        &self.due_date
    }

    pub fn get_category(&self) -> &String {
        &self.category
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn get_priority(&self) -> &i32 {
        &self.priority
    }

    pub fn get_cost(&self) -> &f32 {
        &self.cost
    }

    pub fn get_attr(&self, attr: &str) -> String {
        match attr {
            NAME => self.name.clone(),
            DESCRIPTION => self.description.clone(),
            DATE_ADDED => self.date_added.to_string(),
            DUE_DATE => self.due_date.to_string(),
            CATEGORY => self.category.clone(),
            STATUS => self.status.to_string(),
            PRIORITY => self.priority.to_string(),
            COST => self.cost.to_string(),
            _ => String::new(),
        }
    }

    pub fn get_colored_attr(&self, attr: &str) -> ColoredString {
        match attr {
            NAME => self.name.clone().normal(),
            DESCRIPTION => self.description.clone().normal(),
            DATE_ADDED => self.date_added.to_string().normal(),
            DUE_DATE => self.due_date.to_colored_string(),
            CATEGORY => self.category.clone().normal(),
            STATUS => self.status.to_colored_string(),
            PRIORITY => self.priority.to_string().normal(),
            COST => self.cost.to_string().normal(),
            _ => String::new().normal(),
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {} | {} | {}",
            self.name,
            self.category,
            self.due_date.to_string(),
            self.status.to_string()
        )
    }
}
