use crate::Date;
use crate::Status;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum SortType {
    Name,
    DateAdded,
    DueDate,
    Category,
    Status,
    Priority,
    Cost,
}

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
}

pub fn sort_by_name(arr: &mut Vec<Entry>) {
    arr.sort_by(|a, b| a.name.cmp(&b.name));
}

pub fn sort_by_date_added(arr: &mut Vec<Entry>) {
    arr.sort_by(|a, b| a.date_added.cmp(&b.date_added))
}

pub fn sort_by_due_date(arr: &mut Vec<Entry>) {
    arr.sort_by(|a, b| a.due_date.cmp(&b.due_date))
}

pub fn sort_by_category(arr: &mut Vec<Entry>) {
    arr.sort_by(|a, b| a.category.cmp(&b.category));
}

pub fn sort_by_status(arr: &mut Vec<Entry>) {
    arr.sort_by(|a, b| a.status.cmp(&b.status));
}

pub fn sort_by_priority(arr: &mut Vec<Entry>) {
    arr.sort_by(|a, b| a.priority.cmp(&b.priority));
}

pub fn sort_by_cost(arr: &mut Vec<Entry>) {
    arr.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());
}

pub fn sort(arr: &mut Vec<Entry>, sort_type: SortType) {
    match sort_type {
        SortType::Name => sort_by_name(arr),
        SortType::DateAdded => sort_by_date_added(arr),
        SortType::DueDate => sort_by_due_date(arr),
        SortType::Category => sort_by_category(arr),
        SortType::Status => sort_by_status(arr),
        SortType::Priority => sort_by_priority(arr),
        SortType::Cost => sort_by_cost(arr),
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {} | {} | {}",
            self.name,
            self.category,
            self.due_date.to_colored_string(),
            self.status.to_colored_string()
        )
    }
}
