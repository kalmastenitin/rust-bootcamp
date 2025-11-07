use std::fs::{self, File};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!("\n To-Do List Menu:");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Tasks as Complete");
        println!("4. Delete Task");
        println!("5. Exit");

        let choice = get_input("What you want to do? :");

        match choice.trim(){
            "1" => add_task(&mut tasks),
            "2" => view_task(&tasks),
            "3" => mark_task_complete(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_task(&tasks);
                println!("Tasks saved! Thank You!");
                return;
            }
            _ => println!("Invalid choice please try again."),
        }

    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("❌ Failed to read input");

    input
}

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn save_task(tasks: &Vec<Task>){
    let json = serde_json::to_string_pretty(tasks).expect("❌ Failed to serialize tasks.");
    let mut file = File::create("tasks.json").expect("❌ Failed to save task.");
    file.write_all(json.as_bytes()).expect("❌ Failed to write tasks to file.");
}

fn add_task(tasks: &mut Vec<Task>){
    let description = get_input("Enter task description: ");
    let id = tasks.len() + 1;
    tasks.push(Task{
        id,
        description: description.trim().to_string(),
        completed: false,
    });

    println!("✅ Task added!");
}

fn view_task(tasks: &Vec<Task>){
    if tasks.is_empty(){
        println!("No tasks found.");
    } else {
        for task in tasks {
            let status = if task.completed {"✅"} else {"❌"};
            println!("{} - {}: {}", task.id, status, task.description);
        }
    }
}

fn mark_task_complete(tasks: &mut Vec<Task>) {
    let id = get_input("Enter task ID to mark as complete: ");
    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("Task marked as complete! ✅");
        } else {
            println!("❌ Task Not found!");
        }
    } else {
        println!("❌ Invalid task ID. ");
    }
}

fn delete_task(tasks: &mut Vec<Task>){
    let id = get_input("Enter task ID to delete: ");
    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(index) = tasks.iter().position(|t|t.id == id){
            tasks.remove(index);
            println!("🗑️ Task Deleted!");
        }
         else {
            println!("❌ Task Not found!");
        }
    } else {
        println!("❌ Invalid task ID. ");
    
    }
}