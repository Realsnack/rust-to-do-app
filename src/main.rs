pub mod task;
pub mod task_list;

use std::io::{self, Write};
use std::str::FromStr;

use task::Task;
use task::TaskStatus;
use task_list::SupportedCommand;
use task_list::TaskList;

const CLEAR_SCREEN: &str = "\x1B[2J";

fn main() {
    if cfg!(debug_assertions) {
        println!("Debugging enabled");
    } else {
        println!("{}", CLEAR_SCREEN);
    }

    let list_of_tasks = TaskList::new();
    // println!("{}", CLEAR_SCREEN);
    println!(
        "Create list of tasks with {} tasks",
        list_of_tasks.tasks.len()
    );

    let command = command_selection();
    println!("Selected command: {:?}", command.to_string());

}

fn command_selection() -> SupportedCommand {
    println!();
    println!("Enter a command:");
    println!("  add - adds a task");
    println!("  list - lists all tasks");
    println!("  update - updates a task");
    println!("  delete - deletes a task");
    println!();

    // Read user input
    let mut command = String::new();
    print!("> ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    // Trim string and convert to lowercase
    command = command.trim().to_string();
    command = command.to_lowercase();

    let command_enum = task_list::SupportedCommand::from_str(&command);

    if command_enum.is_err() {
        println!("Invalid command: {}", command);
        
        return command_selection();
    }

    command_enum.unwrap()
}
