use clap::{Arg, Command};
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};

const FILE_PATH : &str = "tasks.txt";

fn main() {

    let matches = Command::new("Task Manager")
        .version("1.0")
        .author("Success Aje <successaje7@gmail.com>")
        .about("CLI tool to manage tasks")

        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(
                    Arg::new("task")
                        .help("The task description")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List all tasks")
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a task by index")
                .arg(
                    Arg::new("index")
                        .help("The index of the task to remove")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let task = sub_m.get_one::<String>("task").unwrap();
            add_task(task);
        }
        Some(("list", _)) => {
            list_tasks();
        }
        Some(("remove", sub_m)) => {
            let index: usize = sub_m.get_one::<String>("index").unwrap().parse().unwrap();
            remove_task(index);
        }
        _ => println!("Unknown command")
    }
}

fn add_task(task : &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILE_PATH)
        .expect("Failed to open or create tasks file");
    writeln!(file, "{}", task).expect("Failed to write task to file");
    println!("Adding task: {}", task);
}

fn list_tasks() {
    let file = OpenOptions::new()
        .read(true)
        .open(FILE_PATH)
        .expect("Failed to open tasks file");

    let reader = io::BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        println!("{} : {}", index, line);
    }
    println!("Listing all tasks");
}

fn remove_task(index: usize) {
let file = OpenOptions::new()
    .read(true)
    .open(FILE_PATH)
    .expect("Failed to open tasks file");

let reader = io::BufReader::new(file);
let tasks : Vec<String> = reader
    .lines()
    .map(|line| line.expect("Failed to read line"))
    .collect();

    if index >= tasks.len() {
        println!("Invalid task index");
        return;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("Failed to open task file for writing");

    for (i, task) in tasks.iter().enumerate() {
        if i != index {
            writeln!(file, "{}", task).expect("Failed to write task");
        }
    }

    println!("Removing task at index: {}" , index)
}



