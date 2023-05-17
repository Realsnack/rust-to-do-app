use crate::task::Task;

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