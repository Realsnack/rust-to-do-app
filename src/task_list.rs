use std::str::FromStr;

use crate::task::Task;
use crate::task::TaskStatus;

pub enum SupportedCommand {
    Add,
    List,
    Update,
    Delete,
    Help,
}

impl SupportedCommand {
    pub fn to_string(&self) -> String {
        match self {
            SupportedCommand::Add => String::from("add"),
            SupportedCommand::List => String::from("list"),
            SupportedCommand::Update => String::from("update"),
            SupportedCommand::Delete => String::from("delete"),
            SupportedCommand::Help => String::from("help"),
        }
    }
}

impl FromStr for SupportedCommand {
    type Err = ();

    fn from_str(input: &str) -> Result<SupportedCommand, Self::Err> {
        match input {
            "add" => Ok(SupportedCommand::Add),
            "list" => Ok(SupportedCommand::List),
            "update" => Ok(SupportedCommand::Update),
            "delete" => Ok(SupportedCommand::Delete),
            "help" => Ok(SupportedCommand::Help),
            _ => Err(()),
        }
    }
}

pub struct TaskList {
    pub tasks: Vec<Task>,
    task_counter: usize,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList { tasks: Vec::new(), task_counter: 0 }
    }

    pub fn add_task(&mut self, title: String, description: Option<String>, due_date: Option<chrono::DateTime<chrono::Local>>) {
        self.task_counter += 1;

        let task = Task {
            id: self.task_counter,
            title,
            description,
            due_date,
            status: TaskStatus::NotStarted,
        };

        self.tasks.push(task);
    }
}
