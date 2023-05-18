use std::str::FromStr;

use crate::naive_date_time_wrapper::NaiveDateTimeWrapper;
use crate::{task::Task, task_status::TaskStatus};

// Save task to csv using the csv crate
pub fn save_tasks_to_csv(tasks: &Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = csv::Writer::from_path("tasks.csv")?;

    for task in tasks {
        writer.serialize(task)?;
    }

    writer.flush()?;
    Ok(())
}

// TODO: Create load tasks from csv function
pub fn load_tasks_from_csv() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let mut tasks: Vec<Task> = Vec::new();

    let mut reader = csv::Reader::from_path("tasks.csv")?;

    // Read lines, split by comma, and create a task
    for result in reader.records() {
        let record = result?;
        let mut task = Task::new(
            record.get(0).unwrap().parse()?,
            record.get(1).unwrap().parse()?,
        );
        let description:String = record.get(2).unwrap().parse()?;
        let due_date = NaiveDateTimeWrapper::from_str(record.get(3).unwrap());
        let status = TaskStatus::from_str(record.get(4).unwrap());

        task.set_description(description);
        match due_date {
            Ok(due_date) => task.set_due_date(due_date),
            Err(_) => (),
        }

        match status {
            Ok(status) => task.set_status(status),
            Err(_) => task.set_status(TaskStatus::NotStarted),
        }

        tasks.push(task);
    }

    Ok(tasks)
}
