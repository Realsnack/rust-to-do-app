use crate::task::Task;
use crate::task::TaskStatus;

pub struct TaskList {
    pub tasks: Vec<Task>,
    task_counter: usize,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            tasks: Vec::new(),
            task_counter: 0,
        }
    }

    pub fn add_task(
        &mut self,
        title: String,
        description: Option<String>,
        due_date: Option<chrono::DateTime<chrono::Local>>,
    ) {
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
