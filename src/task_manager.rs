use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

pub struct Task {
    description: String,
}

impl Task {
    pub fn new(description: String) -> Task {
        Task { description }
    }

    pub fn to_string(&self) -> String {
        self.description.clone()
    }
}

pub enum Command {
    Add(String),
    Remove(usize),
    View,
}

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        let tasks = match Self::load_tasks() {
            Ok(tasks) => tasks,
            Err(_) => Vec::new(),
        };
        TaskManager { tasks }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
        if let Err(e) = self.save_tasks() {
            println!("Failed to save tasks: {}", e);
        }
    }

    pub fn remove_task(&mut self, index: usize) -> Result<(), String> {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            self.save_tasks().map_err(|e| format!("Failed to save tasks: {}", e))?;
            Ok(())
        } else {
            Err("Index out of range".to_string())
        }
    }

    pub fn view_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            println!("{}. {}", i, task.to_string());
        }
    }

    fn save_tasks(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("tasks.txt")?;
        for task in &self.tasks {
            writeln!(file, "{}", task.description)?;
        }
        Ok(())
    }

    fn load_tasks() -> io::Result<Vec<Task>> {
        let file = File::open("tasks.txt")?;
        let reader = BufReader::new(file);

        reader.lines().map(|line| 
            line.map(|l| Task::new(l.trim().to_string()))
        ).collect()
    }
}