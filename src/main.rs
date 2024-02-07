use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

// Define a struct for your messages/tasks
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u64,
    name: String,
    // Add more fields as needed
}

// Function to write a message to the JSON file
fn enqueue_task(task: Task, file_path: &str) {
    let mut tasks = match read_tasks(file_path) {
        Some(tasks) => tasks,
        None => Vec::new(),
    };

    tasks.push(task); // Clone the task and add it to the vector

    write_tasks(&tasks, file_path);
}

// Function to read tasks from the JSON file
fn read_tasks(file_path: &str) -> Option<Vec<Task>> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return None, // Return None if the file doesn't exist or can't be opened
    };

    serde_json::from_reader(file).ok()
}

// Function to write tasks to the JSON file
fn write_tasks(tasks: &[Task], file_path: &str) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true) // Truncate the file to remove previous contents
        .create(true)
        .open(file_path)
        .expect("Failed to open file for writing");

    serde_json::to_writer(file, tasks).expect("Failed to write tasks to file");
}

// Function to dequeue a task from the JSON file
fn dequeue_task(file_path: &str) -> Option<Task> {
    let mut tasks = match read_tasks(file_path) {
        Some(tasks) => tasks,
        None => return None, // Return None if the file doesn't exist or can't be opened
    };

    if tasks.is_empty() {
        return None; // Return None if there are no tasks
    }

    let task = tasks.remove(0); // Remove and return the first task

    write_tasks(&tasks, file_path); // Write the updated tasks back to the file

    Some(task)
}

fn main() {
    // Enqueue tasks
    let task1 = Task { id: 1, name: "Task 1".to_string() };
    enqueue_task(task1, "tasks.json");

    let task2 = Task { id: 2, name: "Task 2".to_string() };
    enqueue_task(task2, "tasks.json");


    loop {
        match dequeue_task("tasks.json") {
            Some(task) => {
                println!("Dequeued task: {:?}", task);
                // Process the task
            }
            None => {
                println!("No more tasks in the queue.");
                break; // Exit the loop when there are no more tasks
            }
        }
    }

    // // Dequeue and process tasks
    // if let Some(task) = dequeue_task("tasks.json") {
    //     println!("Dequeued task: {:?}", task);
    //     // Process the task
    // } else {
    //     println!("No tasks in the queue.");
    // }
}
