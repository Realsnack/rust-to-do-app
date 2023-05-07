use std::str::FromStr;

#[derive(Debug)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
}

impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::NotStarted => "Not Started".to_string(),
            TaskStatus::InProgress => "In Progress".to_string(),
            TaskStatus::Completed => "Completed".to_string(),
        }
    }
}

impl FromStr for TaskStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<TaskStatus, Self::Err> {
        match input.to_lowercase().trim() {
            "not started" => Ok(TaskStatus::NotStarted),
            "in progress" => Ok(TaskStatus::InProgress),
            "completed" => Ok(TaskStatus::Completed),
            _ => Err(()),
        }
    }
}