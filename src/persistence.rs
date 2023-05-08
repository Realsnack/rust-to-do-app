use std::{fs::File, io::Write};

use bincode::Options;

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

pub fn create_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("config.bin")?;
    let config = bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .serialize(&0u8)?;
    let config_bin = bincode::serialize(&config)?;
    file.write_all(&config_bin)?;
    Ok(())
}