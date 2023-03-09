use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::io::{self, BufRead};

fn main() {
    // Define a shared vector to hold programs and their intervals
    let programs = Arc::new(Mutex::new(vec![
        ("Program A".to_string(), Duration::from_secs(60)),
        ("Program B".to_string(), Duration::from_secs(600 * 60 * 5)),
    ]));

    // Spawn a thread to listen for user input to add new programs
    let programs_clone = programs.clone();
    thread::spawn(move || {
        loop {
            // Read user input
            let stdin = io::stdin();
            let line = stdin.lock().lines().next().unwrap().unwrap();
            let tokens: Vec<&str> = line.split(" ").collect();

            // Parse user input to add new programs
            if tokens[0] == "add" && tokens.len() == 3 {
                let program_name = tokens[1].to_string();
                let interval = match tokens[2].parse::<u64>() {
                    Ok(n) => Duration::from_secs(n),
                    Err(_) => {
                        println!("Invalid interval specified.");
                        continue;
                    }
                };

                let mut programs = programs_clone.lock().unwrap();
                programs.push((program_name, interval));
                println!("Program added.");
            } else {
                println!("Invalid command.");
            }
        }
    });

    // Loop through each program and run it on its specified interval
    loop {
        let programs = programs.lock().unwrap();
        for (program_name, interval) in programs.iter() {
            let program_name_clone = program_name.clone();
            let interval_clone = *interval;
            thread::spawn(move || {
                loop {
                    // Run the program's function
                    run_program(&program_name_clone);

                    // Wait for the specified interval
                    thread::sleep(interval_clone);
                }
            });
        }

        // Wait for all threads to finish
        thread::park_timeout(Duration::from_secs(1));
    }
}

// Function that runs the program
fn run_program(program_name: &str) {
    println!("Running {}...", program_name);
    // Add code here to run the program's functionality
}
