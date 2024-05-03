// mod status;

use crate::Date;
// use status::Status;
use crate::Status;
use serde::{Deserialize, Serialize};

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
