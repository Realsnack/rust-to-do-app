pub mod command;
pub mod naive_date_time_wrapper;
pub mod persistence;
pub mod task;
pub mod task_list;
pub mod task_status;

use colorize::AnsiColor;

use std::{
    io::{self, Write},
    str::FromStr,
};

use command::SupportedCommand;
use task_list::TaskList;

use crate::{task_status::TaskStatus, naive_date_time_wrapper::NaiveDateTimeWrapper};

const CLEAR_SCREEN: &str = "\x1B[2J";

fn main() {
    if cfg!(debug_assertions) {
        println!("Debugging enabled");
    } else {
        println!("{}", CLEAR_SCREEN);
    }

    let mut list_of_tasks = TaskList::new();

    println!(
        "Created list of tasks with {} tasks",
        list_of_tasks.tasks.len()
    );

    loop {
        let command = command_selection();
        println!();

        match command {
            SupportedCommand::Add => add_task(&mut list_of_tasks),
            SupportedCommand::List => list_tasks(&list_of_tasks),
            SupportedCommand::Update => update_task(&mut list_of_tasks),
            SupportedCommand::Delete => delete_task(&mut list_of_tasks),
            SupportedCommand::Help => help(),
            SupportedCommand::Exit => {
                list_of_tasks.save_tasks_to_csv();
                println!("Exiting");
                break;
            }
        }
    }
}

fn command_selection() -> SupportedCommand {
    println!();
    println!("Enter a command:");
    println!("  {} - adds a task", "add".bold().green());
    println!("  {} - lists all tasks", "list".bold().cyan());
    println!("  {} - updates a task", "update".bold().yellow());
    println!("  {} - deletes a task", "delete".bold().red());
    println!("  {} - prints this help message", "help".bold().grey());
    println!("  {} - exits the program", "exit".bold().magenta());
    println!();

    let command = get_user_input("Enter command");
    let command_enum = SupportedCommand::from_str(&command.to_lowercase());

    if command_enum.is_err() {
        println!("Invalid command: {}", command);

        return command_selection();
    }

    command_enum.unwrap()
}

fn get_user_input(prompt_text: &str) -> String {
    println!("{}", prompt_text);
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn add_task(list_of_tasks: &mut TaskList) {
    let mut title;
    loop {
        title = get_user_input("Enter title for task");
        println!();

        if title.is_empty() {
            println!("Title cannot be empty");
            println!();
            continue;
        } else {
            break;
        }
    }

    let task_id = list_of_tasks.add_task(title);

    let description = get_user_input("Enter a description for the task:");
    println!();

    if !description.is_empty() {
        list_of_tasks.update_task_description(task_id, description);
    }

    loop {
        let mut due_date =
            get_user_input("Enter a due date and time for the task (dd.mm.YYYY HH:MM):");
        println!();

        if due_date.is_empty() {
            break;
        }

        due_date += ":00";

        let parsed_date = NaiveDateTimeWrapper::parse_from_str(&due_date, "%d.%m.%Y %H:%M:%S");

        match parsed_date {
            Ok(_) => {
                list_of_tasks.update_task_due_date(task_id, parsed_date.unwrap());
                break;
            }
            Err(_) => {
                println!("Invalid date format, please try again");
                println!("{:?}", parsed_date.unwrap_err());
                println!();
            }
        }
    }

    press_enter();
}

fn list_tasks(list_of_tasks: &TaskList) {
    println!("{}", CLEAR_SCREEN);

    if list_of_tasks.tasks.len() == 0 {
        println!("No tasks to list");
        press_enter();
        return;
    }

    println!("List of tasks:");
    println!();
    for task in &list_of_tasks.tasks {
        println!("  {} - {}", task.id, task.title);
        if task.description.is_some() {
            println!("    Description: {}", task.description.as_ref().unwrap());
        }
        if task.due_date.is_some() {
            println!("    Due date: {}", task.due_date.as_ref().unwrap());
        }
        println!("    Status: {}", task.status.to_string());
        println!();
    }
    press_enter();
}

fn update_task(list_of_tasks: &mut TaskList) {
    if list_of_tasks.tasks.len() == 0 {
        println!("No tasks to update");
        press_enter();
        return;
    }
    let task_id_input = get_user_input("Enter task Id to update");
    // convert task_id to usize
    let task_id_input = task_id_input.parse::<usize>();
    let mut task_id: usize = 0;

    match task_id_input {
        Ok(parsed_task_id) => {
            if task_id > list_of_tasks.tasks.len() {
                println!("Invalid task Id: {}", task_id);
                press_enter();
                return;
            }
            task_id = parsed_task_id;
        }
        Err(_) => {
            println!("Invalid task");
            press_enter();
            return;
        }
    }

    println!("{}", CLEAR_SCREEN);
    // print task and details
    println!(
        "Task: {}",
        list_of_tasks.get_task_by_id(task_id).unwrap().title
    );
    println!(
        "  Description: {}",
        list_of_tasks
            .get_task_by_id(task_id)
            .unwrap()
            .description
            .clone()
            .unwrap_or("Not provided".to_string())
    );
    let due_date = list_of_tasks
        .get_task_by_id(task_id)
        .unwrap()
        .due_date
        .clone();
    if due_date.is_some() {
        println!("  Due date: {}", due_date.unwrap());
    } else {
        println!("  Due date: Not provided");
    }
    println!(
        "  Status: {}",
        list_of_tasks
            .get_task_by_id(task_id)
            .unwrap()
            .status
            .to_string()
    );

    loop {
        let field_to_update = get_user_input("Choose field to update").to_lowercase();
        println!();

        match field_to_update.as_str() {
            "description" => {
                let new_description = get_user_input("Enter new description");
                println!();
                list_of_tasks.update_task_description(task_id, new_description);
                break;
            }
            "due date" => {
                loop {
                    let new_date_string =
                        get_user_input("Enter new due date and time (dd.mm.YYYY HH:MM)");
                    println!();
                    let new_date_string = new_date_string + ":00";
                    let parsed_date = NaiveDateTimeWrapper::parse_from_str(
                        &new_date_string,
                        "%d.%m.%Y %H:%M:%S",
                    );
                    match parsed_date {
                        Ok(_) => {
                            list_of_tasks.update_task_due_date(task_id, parsed_date.unwrap());
                            break;
                        }
                        Err(_) => {
                            println!("Invalid date format, please try again");
                            println!("{:?}", parsed_date.unwrap_err());
                            println!();
                            continue;
                        }
                    }
                }
                break;
            }
            "status" => {
                println!(
                    "Available statuses: {:?}, {:?}, {:?}",
                    TaskStatus::NotStarted.to_string(),
                    TaskStatus::InProgress.to_string(),
                    TaskStatus::Completed.to_string()
                );
                let new_status = get_user_input("Enter new status");
                println!();
                let new_status = new_status.parse::<TaskStatus>();
                match new_status {
                    Ok(parsed_status) => {
                        list_of_tasks.update_task_status(task_id, parsed_status);
                        break;
                    }
                    Err(_) => {
                        println!("Invalid status, please try again");
                        continue;
                    }
                }
            }
            _ => {
                println!("Invalid field to update: {}", field_to_update);
                println!();
                continue;
            }
        }
    }

    press_enter();
}

fn delete_task(list_of_tasks: &mut TaskList) {
    if list_of_tasks.tasks.len() == 0 {
        println!("No tasks to delete");
        press_enter();
        return;
    }

    if list_of_tasks.tasks.len() == 0 {
        println!("No tasks to update");
        press_enter();
        return;
    }
    let task_id_input = get_user_input("Enter task Id to update");
    // convert task_id to usize
    let task_id_input = task_id_input.parse::<usize>();
    let mut task_id: usize = 0;

    match task_id_input {
        Ok(parsed_task_id) => {
            if task_id > list_of_tasks.tasks.len() {
                println!("Invalid task Id: {}", task_id);
                press_enter();
                return;
            }
            task_id = parsed_task_id;
        }
        Err(_) => {
            println!("Invalid task");
            press_enter();
            return;
        }
    }

    println!("{}", CLEAR_SCREEN);
    let deletion_result = list_of_tasks.delete_task(task_id);

    match deletion_result {
        Ok(_) => {
            println!("Task deleted successfully");
        }
        Err(_) => {
            println!("Failed to delete task");
        }
    }

    press_enter();
}

fn help() {
    println!("{}", CLEAR_SCREEN);
    println!("To-Do app by @realsnack");
    println!("https://github.com/Realsnack/rust-to-do-app");
    println!();
    println!();
    println!("You can use these commands:");
    println!("  {} - Allows you to add a new task", "add".bold().green());
    println!("    The application will first ask you to enter a title for the task");
    println!("    Then it will ask you to enter a description for the task - if you don't want to enter a description, just press enter");
    println!("    Finally it will ask you to enter a due date for the task - if you don't want to enter a due date, just press enter");
    println!("    The task is automatically created with status 'Not started'");
    println!("  {} - list all tasks", "list".bold().cyan());
    println!("    This will print a list of all tasks");
    println!("    Each task will have an ID, title, description, and due date. If a task doesn't have a description or due date, it will be marked as such");
    println!(
        "  {} - allows you to update a task",
        "update".bold().yellow()
    );
    println!("    This will allow you to update a task");
    println!("    You will first be asked to enter the ID of the task you want to update. Then you will be asked to select which field you want to update. Finally you will be asked to enter the new value for the field");
    println!("  {} - deletes a task", "delete".bold().red());
    println!("    This will allow you to delete a task");
    println!("    You will first be asked to enter the ID of the task you want to delete, then a confirmation message will be displayed. If you confirm, the task will be deleted");
    println!("  {} - prints this help message", "help".bold().grey());
    println!("  {} - exits the program", "exit".bold().magenta());
    println!();
    press_enter();

    println!("{}", CLEAR_SCREEN);
}

fn press_enter() {
    print!("Press enter to continue");
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
}
