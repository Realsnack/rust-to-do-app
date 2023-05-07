pub mod command;
pub mod task;
pub mod task_list;

use colorize::AnsiColor;

use std::{
    io::{self, Write},
    str::FromStr,
};

use command::SupportedCommand;
use task_list::TaskList;

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

    let command = get_user_input();
    let command_enum = SupportedCommand::from_str(&command.to_lowercase());

    if command_enum.is_err() {
        println!("Invalid command: {}", command);

        return command_selection();
    }

    command_enum.unwrap()
}


fn get_user_input() -> String {
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
        println!("Enter a title for the task:");
        title = get_user_input();
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

    println!("Enter a description for the task:");
    let description = get_user_input();
    println!();

    if !description.is_empty() {
        list_of_tasks.update_task_description(task_id, description);
    }

    loop {
        println!("Enter a due date and time for the task (dd.mm.YYYY HH:MM):");
        let mut due_date = get_user_input();
        println!();
        
        if due_date.is_empty() {
            break;
        }
        
        due_date += ":00";
        
        let parsed_date = chrono::NaiveDateTime::parse_from_str(&due_date, "%d.%m.%Y %H:%M:%S");
        
        match parsed_date {
            Ok(_) => {
                list_of_tasks.update_task_due_date(task_id, parsed_date.unwrap());
                break;
            },
            Err(_) => {
                println!("Invalid date format, please try again");
                println!("{}", parsed_date.unwrap_err());
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

// TODO: Update task
fn update_task(list_of_tasks: &mut TaskList) {
    println!("Update a task");

    // 1. select task Id
    // 2. select field to update
    // 3. update task
}

// TODO: Delete task
fn delete_task(list_of_tasks: &mut TaskList) {
    println!("Delete a task");

    // 1. select task Id
    // 2. delete task
}

fn help() {
    println!("{}", CLEAR_SCREEN);
    println!("To-Do app by @realsnack");
    println!("https://github.com/Realsnack/rust-to-do-app");
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
    println!("  {} - allows you to update a task", "update".bold().yellow());
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
