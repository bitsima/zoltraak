use rand::Rng;
use reqwest::Client;
use std::{fs::File, io::Read, path::Path, thread, time::Duration};

const MIN_CHUNK_SIZE: usize = 256 * 1024; // 256 KB
const MAX_CHUNK_SIZE: usize = 1024 * 1024; // 1 MB

pub async fn send_file(file_path: &Path, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut file = File::open(file_path)?;
    let mut chunk_number = 0;

    let mut rng = rand::thread_rng();

    loop {
        // Generate a random chunk size within the specified range
        let chunk_size = rng.gen_range(MIN_CHUNK_SIZE..=MAX_CHUNK_SIZE);
        let mut buffer = vec![0; chunk_size];

        match file.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                let response = client
                    .post(url)
                    .header("Chunk-Number", chunk_number.to_string())
                    .body(buffer[..bytes_read].to_vec())
                    .send()
                    .await?;

                if !response.status().is_success() {
                    eprintln!("Failed to send chunk {}", chunk_number);
                }

                chunk_number += 1;

                let seconds_to_sleep = rand::thread_rng().gen_range(2..11);
                thread::sleep(Duration::from_secs(seconds_to_sleep));
            }
            Ok(_) => break, // End of file
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(())
}
