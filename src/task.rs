use crate::TaskStatus;

use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<NaiveDateTime>,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(id: usize, title: String) -> Task {
        Task {
            id,
            title,
            description: None,
            due_date: None,
            status: TaskStatus::NotStarted,
        }
    }
    
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn set_due_date(&mut self, due_date: NaiveDateTime) {
        self.due_date = Some(due_date);
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }
}
