use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI To-Do List", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { task: String },
    /// List all tasks
    List,
    /// Mark a task as done
    Done { id: usize },
    /// Remove a task
    Remove { id: usize },
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

fn main() {
    let cli = Cli::parse();
    let file_path = PathBuf::from("tasks.json");
    
    let mut tasks = load_tasks(&file_path);

    match &cli.command {
        Commands::Add { task } => {
            tasks.push(Task {
                description: task.clone(),
                completed: false,
            });
            save_tasks(&file_path, &tasks);
            println!("{} Added: {}", "SUCCESS:".green().bold(), task);
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("{}", "No tasks found!".yellow());
            } else {
                println!("{}", "--- Your Tasks ---".cyan().bold());
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.completed {
                        "[X]".green()
                    } else {
                        "[ ]".red()
                    };
                    println!("{} {}: {}", i + 1, status, task.description);
                }
            }
        }
        Commands::Done { id } => {
            if let Some(task) = tasks.get_mut(id - 1) {
                task.completed = true;
                save_tasks(&file_path, &tasks);
                println!("{} Marked task {} as done!", "SUCCESS:".green().bold(), id);
            } else {
                println!("{} Task ID {} not found.", "ERROR:".red().bold(), id);
            }
        }
        Commands::Remove { id } => {
            if *id > 0 && *id <= tasks.len() {
                let removed = tasks.remove(id - 1);
                save_tasks(&file_path, &tasks);
                println!("{} Removed: {}", "SUCCESS:".green().bold(), removed.description);
            } else {
                println!("{} Task ID {} not found.", "ERROR:".red().bold(), id);
            }
        }
    }
}

fn load_tasks(path: &PathBuf) -> Vec<Task> {
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

fn save_tasks(path: &PathBuf, tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write(path, data).expect("Failed to write tasks to file");
}
