use crate::Entry;
use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::fs::OpenOptions;

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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TodoList {
    entries: Vec<Entry>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            entries: Vec::new(),
        }
    }

    pub fn from_entry(entry: Entry) -> TodoList {
        TodoList {
            entries: vec![entry],
        }
    }

    pub fn from_vec(entries: Vec<Entry>) -> TodoList {
        TodoList { entries }
    }

    pub fn from_file(file_name: &str) -> TodoList {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true) // Create the file if it doesn't exist
            .open(file_name)
            .expect("Failed to open file");
        // let file = std::fs::File::open(file_name).create(file_name).expect("Failed to open file");
        let reader = std::io::BufReader::new(file);
        let entries: Vec<Entry> = serde_json::from_reader(reader).unwrap();
        TodoList { entries }
    }

    pub fn to_file(&self, file_name: &str) {
        let file = std::fs::File::create(file_name).expect("Failed to create file");
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, &self.entries).expect("Failed to write to file");
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn add_entries(&mut self, entries: Vec<Entry>) {
        self.entries.extend(entries);
    }

    pub fn remove_entry(&mut self, index: usize) {
        self.entries.remove(index);
    }

    pub fn remove_entries(&mut self, indices: Vec<usize>) {
        for index in indices.iter() {
            self.entries.remove(*index);
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn update_entry(&mut self, index: usize, entry: Entry) {
        self.entries[index] = entry;
    }

    pub fn get_entry(&self, index: usize) -> &Entry {
        &self.entries[index]
    }

    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn sort_by_name(&mut self) {
        self.entries.sort_by(|a, b| a.get_name().cmp(&b.get_name()));
    }

    pub fn sort_by_date_added(&mut self) {
        self.entries
            .sort_by(|a, b| a.get_date_added().cmp(&b.get_date_added()))
    }

    pub fn sort_by_due_date(&mut self) {
        self.entries
            .sort_by(|a, b| a.get_due_date().cmp(&b.get_due_date()))
    }

    pub fn sort_by_category(&mut self) {
        self.entries
            .sort_by(|a, b| a.get_category().cmp(&b.get_category()));
    }

    pub fn sort_by_status(&mut self) {
        self.entries
            .sort_by(|a, b| a.get_status().cmp(&b.get_status()));
    }

    pub fn sort_by_priority(&mut self) {
        self.entries
            .sort_by(|a, b| a.get_priority().cmp(&b.get_priority()));
    }

    pub fn sort_by_cost(&mut self) {
        self.entries
            .sort_by(|a, b| a.get_cost().partial_cmp(&b.get_cost()).unwrap());
    }

    pub fn sort(&mut self, sort_type: SortType) {
        match sort_type {
            SortType::Name => self.sort_by_name(),
            SortType::DateAdded => self.sort_by_date_added(),
            SortType::DueDate => self.sort_by_due_date(),
            SortType::Category => self.sort_by_category(),
            SortType::Status => self.sort_by_status(),
            SortType::Priority => self.sort_by_priority(),
            SortType::Cost => self.sort_by_cost(),
        }
    }

    pub fn print_entries(&self, attrs: &[&str]) {
        if self.entries.is_empty() {
            println!("Nothing to do...");
            return;
        }
        if attrs.is_empty() {
            println!("No attributes to display...");
            return;
        }
        let mut maximums = Vec::with_capacity(attrs.len());
        for attr in attrs.iter() {
            let max_len = self
                .entries
                .iter()
                .map(|entry| entry.get_attr(attr).len())
                .max()
                .unwrap();
            maximums.push(max(max_len, attr.len()));
        }

        let mut header = String::new();
        let mut separator = String::new();
        for (i, attr) in attrs.iter().enumerate() {
            header.push_str(&format!("{:<max_len$} | ", attr, max_len = maximums[i]));
            separator.push_str(&format!("{:-<max_len$}+-", "", max_len = maximums[i] + 1));
        }
        header.pop();
        separator.pop();
        println!("{}", header);
        println!("{}", separator);
        for (i, entry) in self.entries.iter().enumerate() {
            let mut line = String::new();
            for (j, attr) in attrs.iter().enumerate() {
                line.push_str(&format!(
                    "{:<max_len$} | ",
                    entry.get_colored_attr(attr),
                    max_len = maximums[j],
                ));
            }
            line.push_str(&format!("{}", i));
            println!("{}", line);
        }
        println!("{}", separator);
    }
}
