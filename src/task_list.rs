use crate::task::Task;
use crate::task::TaskStatus;

// THINK: Maybe use a hashmap instead of a vector?
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

    pub fn add_task(&mut self, title: String) -> usize {
        self.task_counter += 1;

        let task = Task {
            id: self.task_counter,
            title,
            description: None,
            due_date: None,
            status: TaskStatus::NotStarted,
        };

        self.tasks.push(task);

        self.task_counter
    }

    pub fn update_task_description(&mut self, task_id: usize, description: String) {
        let task = self.tasks.iter_mut().find(|t| t.id == task_id);

        if let Some(task) = task {
            task.set_description(description);
        }
    }
}
