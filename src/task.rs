use serde_derive::Serialize;

use crate::TaskStatus;
use crate::naive_date_time_wrapper::NaiveDateTimeWrapper;

#[derive(Debug, Serialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<NaiveDateTimeWrapper>,
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

    pub fn set_due_date(&mut self, due_date: NaiveDateTimeWrapper) {
        self.due_date = Some(due_date);
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }
}

impl Clone for Task {
    fn clone(&self) -> Self {
        Task {
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            due_date: self.due_date,
            status: self.status.clone(),
        }
    }
}
