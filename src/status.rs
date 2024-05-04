use crate::colors::{DONE_COLOR, IN_PROGRESS_COLOR};

use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Done,
    InProgress,
    NotStarted,
}

impl Status {
    pub fn to_string(&self) -> String {
        match self {
            Status::Done => "Done".to_string(),
            Status::InProgress => "In Progress".to_string(),
            Status::NotStarted => "Not Started".to_string(),
        }
    }
    pub fn to_colored_string(&self) -> ColoredString {
        match self {
            Status::Done => self.to_string().custom_color(DONE_COLOR),
            Status::InProgress => self.to_string().custom_color(IN_PROGRESS_COLOR),
            Status::NotStarted => self.to_string().normal(),
        }
    }
}
