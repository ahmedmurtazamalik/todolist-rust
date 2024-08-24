use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufReader, Write};

#[derive(Serialize, Deserialize)]
struct TodoList {
    pending_tasks: Vec<String>,
    completed_tasks: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct TodoManager {
    lists: HashMap<String, TodoList>,
}

impl TodoManager {
    fn new() -> Self {
        TodoManager {
            lists: HashMap::new(),
        }
    }

    fn load(filename: &str) -> io::Result<Self> {
        let file = File::open(filename).ok();
        if let Some(file) = file {
            let reader = BufReader::new(file);
            let manager = serde_json::from_reader(reader).unwrap_or_else(|_| Self::new());
            Ok(manager)
        } else {
            Ok(Self::new())
        }
    }

    fn save(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }

    fn add_list(&mut self, title: String) {
        self.lists.insert(title, TodoList {
            pending_tasks: Vec::new(),
            completed_tasks: Vec::new(),
        });
    }

    fn add_task(&mut self, list: &str, task: String) {
        if let Some(todo_list) = self.lists.get_mut(list) {
            todo_list.pending_tasks.push(task);
        } else {
            println!("List '{}' not found!", list);
        }
    }

    fn delete_task(&mut self, list: &str, task_index: usize) {
        if let Some(todo_list) = self.lists.get_mut(list) {
            if task_index < todo_list.pending_tasks.len() {
                todo_list.pending_tasks.remove(task_index);
            } else {
                println!("Task index out of bounds!");
            }
        } else {
            println!("List '{}' not found!", list);
        }
    }

    fn cross_off_task(&mut self, list: &str, task_index: usize) {
        if let Some(todo_list) = self.lists.get_mut(list) {
            if task_index < todo_list.pending_tasks.len() {
                let task = todo_list.pending_tasks.remove(task_index);
                todo_list.completed_tasks.push(task);
            } else {
                println!("Task index out of bounds!");
            }
        } else {
            println!("List '{}' not found!", list);
        }
    }

    fn view_list(&self, list: &str) {
        if let Some(todo_list) = self.lists.get(list) {
            println!("\nPending tasks:");
            for (index, task) in todo_list.pending_tasks.iter().enumerate() {
                println!("{}. {}", index + 1, task);
            }
            println!("\nCompleted tasks:");
            for (index, task) in todo_list.completed_tasks.iter().enumerate() {
                println!("{}. {}", index + 1, task);
            }
        } else {
            println!("List '{}' not found!", list);
        }
    }

    fn select_list(&self) -> Option<String> {
        if self.lists.is_empty() {
            println!("No lists available.");
            return None;
        }

        println!("\nAvailable lists:");
        let mut keys: Vec<&String> = self.lists.keys().collect();
        for (index, key) in keys.iter().enumerate() {
            println!("{}. {}", index + 1, key);
        }

        print!("Enter the number of the list to select: ");
        io::stdout().flush().unwrap();
        let mut list_index = String::new();
        io::stdin().read_line(&mut list_index).unwrap();
        let list_index: usize = list_index.trim().parse().unwrap_or(0);

        if list_index == 0 || list_index > keys.len() {
            println!("Invalid list number!");
            return None;
        }

        Some(keys.remove(list_index - 1).to_string())
    }

    fn select_task(&self, list: &str) -> Option<usize> {
        if let Some(todo_list) = self.lists.get(list) {
            if todo_list.pending_tasks.is_empty() {
                println!("No pending tasks available in the list '{}'.", list);
                return None;
            }

            println!("\nPending tasks:");
            for (index, task) in todo_list.pending_tasks.iter().enumerate() {
                println!("{}. {}", index + 1, task);
            }

            print!("Enter the number of the task: ");
            io::stdout().flush().unwrap();
            let mut task_index = String::new();
            io::stdin().read_line(&mut task_index).unwrap();
            let task_index: usize = task_index.trim().parse().unwrap_or(0);

            if task_index == 0 || task_index > todo_list.pending_tasks.len() {
                println!("Invalid task number!");
                return None;
            }

            Some(task_index - 1)
        } else {
            println!("List '{}' not found!", list);
            None
        }
    }
}

fn main() {
    let mut manager = TodoManager::load("todo.json").unwrap();

    loop {
        println!("\n--- To-Do List Manager ---");
        println!("1. Add a new list");
        println!("2. Add a task to a list");
        println!("3. Delete a task from a list");
        println!("4. Cross off a task from a list");
        println!("5. View a list");
        println!("6. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter the title of the new list: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim().to_string();
                manager.add_list(title);
                println!("List added successfully!");
            }
            "2" => {
                if let Some(list) = manager.select_list() {
                    print!("Enter the task: ");
                    io::stdout().flush().unwrap();
                    let mut task = String::new();
                    io::stdin().read_line(&mut task).unwrap();
                    let task = task.trim().to_string();
                    manager.add_task(&list, task);
                    println!("Task added successfully!");
                }
            }
            "3" => {
                if let Some(list) = manager.select_list() {
                    if let Some(task_index) = manager.select_task(&list) {
                        manager.delete_task(&list, task_index);
                        println!("Task deleted successfully!");
                    }
                }
            }
            "4" => {
                if let Some(list) = manager.select_list() {
                    if let Some(task_index) = manager.select_task(&list) {
                        manager.cross_off_task(&list, task_index);
                        println!("Task crossed off successfully!");
                    }
                }
            }
            "5" => {
                if let Some(list) = manager.select_list() {
                    manager.view_list(&list);
                }
            }
            "6" => {
                manager.save("todo.json").unwrap();
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please try again."),
        }
    }
}
