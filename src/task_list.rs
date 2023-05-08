use crate::naive_date_time_wrapper::NaiveDateTimeWrapper;
use crate::task::Task;
use crate::TaskStatus;

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

    pub fn update_task_due_date(&mut self, task_id: usize, due_date: NaiveDateTimeWrapper) {
        let task = self.tasks.iter_mut().find(|t| t.id == task_id);

        if let Some(task) = task {
            task.set_due_date(due_date);
        }
    }

    pub fn update_task_status(&mut self, task_id: usize, status: TaskStatus) {
        let task = self.tasks.iter_mut().find(|t| t.id == task_id);

        if let Some(task) = task {
            task.set_status(status);
        }
    }

    pub fn delete_task(&mut self, task_id: usize) -> Result<(), ()> {
        let task_index = self.tasks.iter().position(|t| t.id == task_id);

        if let Some(task_index) = task_index {
            self.tasks.remove(task_index);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_task_by_id(&self, task_id: usize) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == task_id)
    }
}
