mod task_manager;

use std::io::{self, Write};
use task_manager::{Task, Command};

fn main() -> io::Result<()> {
    let mut manager = task_manager::TaskManager::new();

    loop {
        println!("\n--- To-Do List ---");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Remove Task");
        println!("4. Exit");

        let mut choice = String::new();
        print!("Enter your choice: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush()?;
                let mut task_desc = String::new();
                io::stdin().read_line(&mut task_desc)?;
                manager.add_task(Task::new(task_desc.trim().to_string()));
            },
            "2" => {
                manager.view_tasks();
            },
            "3" => {
                print!("Enter task index to remove: ");
                io::stdout().flush()?;
                let mut index = String::new();
                io::stdin().read_line(&mut index)?;
                if let Ok(index) = index.trim().parse::<usize>() {
                    if let Err(e) = manager.remove_task(index) {
                        println!("Error: {}", e);
                    }
                } else {
                    println!("Please enter a valid number.");
                }
            },
            "4" => {
                println!("Exiting program.");
                break;
            },
            _ => println!("Invalid choice."),
        }
    }
    Ok(())
}