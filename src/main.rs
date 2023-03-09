use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    // Read program intervals from a file
    let intervals = read_intervals_from_file("./program_intervals.txt").unwrap();

    // Start a thread for each program
    let mut threads = vec![];
    for (program, url, method, parameter, interval) in intervals {
        let thread = thread::spawn(move || {
            loop {
                let start_time = SystemTime::now();
                // Execute the program
                execute_program(&program, &url, &method, &parameter);

                // Wait until the next interval
                let elapsed_time = start_time.elapsed().unwrap().as_secs();
                let remaining_time = interval - elapsed_time % interval;
                thread::sleep(Duration::from_secs(remaining_time));
            }
        });
        threads.push(thread);
    }

    // Wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }
}

fn read_intervals_from_file(filename: &str) -> Result<Vec<(String, String, String, String, u64)>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut intervals = vec![];
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 5 {
            let program = parts[0].trim().to_string();
            let url = parts[1].trim().to_string();
            let method = parts[2].trim().to_string();
            let parameter = parts[3].trim().to_string();
            let interval = parts[4].trim().parse().unwrap();
            intervals.push((program, url, method, parameter, interval));
        }
    }
    Ok(intervals)
}

fn execute_program(program: &String, x: &String, x0: &String, x1: &String) {
    // Execute the program here
    println!("Executing program: {}", program);
}
