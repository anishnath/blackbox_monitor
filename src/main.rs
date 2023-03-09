use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::{Duration, SystemTime};
use reqwest::{Error, Client};
use serde_json::{json, Value};

fn main() {
    // Read program intervals from a file
    let intervals = read_intervals_from_file("./program_intervals.txt").unwrap();

    // Start a thread for each program
    let mut threads = vec![];
    for (program, url, method, parameter, timeout, interval) in intervals {
        let thread = thread::spawn(move || {
            loop {
                let start_time = SystemTime::now();
                // Execute the program
                execute_program(&program, &url, &method, &parameter,  &timeout);

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

fn read_intervals_from_file(filename: &str) -> Result<Vec<(String, String, String, String, u64, u64)>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut intervals = vec![];
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 6 {
            let program = parts[0].trim().to_string();
            let url = parts[1].trim().to_string();
            let method = parts[2].trim().to_string();
            let parameter = parts[3].trim().to_string();
            let timeout = parts[4].trim().parse().unwrap();
            let interval = parts[5].trim().parse().unwrap();
            intervals.push((program, url, method, parameter, timeout,  interval));
        }
    }
    Ok(intervals)
}

fn execute_program(program: &String, url: &String, method: &String, parameter: &String, timeout: &u64) {

    println!(
        "Executing program: {} {} {} {} {}",
        program, url, method, parameter, timeout
    );
    // If method is GET, perform an HTTP GET request
    if method.to_uppercase() == "GET" {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        match rt.block_on(perform_get_request(url, timeout)) {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Successfully performed GET request. Response body:");
                } else {
                    println!("Error performing GET request. Status code: {}", response.status());
                    if let Err(e) = send_webhook(webhook_url, &response).await {
                        println!("Error sending webhook: {}", e);
                    }
                }
            },
            Err(e) => println!("Error performing GET request: {}", e),
        }
    }
}


async fn perform_get_request(url: &String, timeout: &u64) -> Result<reqwest::Response, reqwest::Error> {
    // Create a new reqwest client
    let client = reqwest::Client::new();

    // Send an HTTP GET request to the specified URL
    let response = client
        .get(url)
        .timeout(Duration::from_secs(*timeout))
        .send()
        .await?;

    Ok(response)
}

// async fn send_discord_webhook(program: &str, url: &str, method: &str, timeout: &u64) -> Result<(), Error> {
//     let webhook_url = "https://discord.com/api/webhooks/1234567890123456789/AbCdEfGhIjKlMnOpQrStUvWxYz";
//     let message = "Hello, world!";
//     let client = Client::new();
//     let payload = json!({ "content": message });
//     let res = client.post(webhook_url)
//         .json(&payload)
//         .send()
//         .await?;
//     let response_text = response.text().await?;
//     let response_json: Value = serde_json::from_str(&response_text)
//         .map_err(|err| reqwest::Error::from(err))?;
//     if response_json["code"] == 204 {
//         Ok(())
//     } else {
//         //println!("Sorry cant post ")
//         //Err(Error::from(format!("Discord webhook returned code {}", response_json["code"])))
//         Err(reqwest::Error::from(format!("Discord webhook returned code {}", response_json["code"])))
//
//     }
// }

fn send_webhook(webhook_url: &str, response: &reqwest::Response) -> Result<(), reqwest::Error> {
    let status_code = response.status();
    let response_body = response.text()?;
    let message = format!("Error performing GET request. Status code: {}. Response body: {}", status_code, response_body);
    let client = Client::new();
    client.post(webhook_url).json(&message).send()?;
    Ok(())
}