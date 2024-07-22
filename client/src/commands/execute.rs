use std::path::Path;
use std::process::Command;
use uuid::Uuid;

use crate::commands::{download, upload};

const DEFAULT_SYSINFO_FILE: &str = "sysinfo.dat";

pub async fn execute_command(
    uuid: Uuid,
    cmd: &str,
    upload_url: &str,
    download_url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let cmd_parts = cmd.split_ascii_whitespace().collect::<Vec<&str>>();
    let cmd_type = cmd_parts.first().unwrap_or(&"");
    let mut output = String::new();

    match *cmd_type {
        "shell" => output = execute_shell_command(cmd_parts)?,
        // upload <file_name>(optional)
        "upload" => output = execute_upload_command(uuid, cmd_parts, upload_url).await?,
        // download <file_id> <file_save_path>
        "download" => output = execute_download_command(uuid, cmd_parts, download_url).await?,
        // "kill" => execute_kill_command(cmd_parts),
        // "start" => execute_start_command(cmd_parts),
        // "restart" => execute_restart_command(cmd_parts),
        _ => {
            println!("Unsupported command type: {}", cmd_type);
            return Err(format!("Unsupported command type: {}", cmd_type).into());
        }
    }

    Ok(output)
}

fn execute_shell_command(cmd_parts: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    if cmd_parts.len() <= 1 {
        println!("Could not execute shell command.");
        return Err(format!("Wrong number of parameters provided").into());
    }

    let shell_command = &cmd_parts[1..].join(" ");

    let output = Command::new("sh")
        .arg("-c")
        .arg(shell_command)
        .output()
        .map_err(|e| format!("Failed to execute process: {}", e))?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("Command output: {}", output_str);
        Ok(output_str.to_string())
    } else {
        let error_str = String::from_utf8_lossy(&output.stderr);
        println!("Command failed with error: {}", error_str);
        Ok(error_str.into())
    }
}

/**
 * Function to execute "upload" commands, will upload the default sysinfo file if the file name is not provided
 */
async fn execute_upload_command(
    uuid: Uuid,
    cmd: Vec<&str>,
    file_url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("Executing upload command: {}", cmd.join(" "));
    let mut file_to_send = String::new();

    if cmd.len() == 1 {
        // default file is sent
        file_to_send = DEFAULT_SYSINFO_FILE.to_string();
    } else if cmd.len() == 2 {
        // given file is sent
        file_to_send = cmd.get(1).unwrap().to_string();
    } else {
        println!("Could not execute upload command.");
        return Err(format!("Wrong number of parameters provided").into());
    }
    if let Err(e) = upload::send_file(uuid, Path::new(&file_to_send), file_url).await {
        eprintln!("Error sending file: {}", e);
        return Err(e);
    }
    Ok(format!(
        "File '{}' was uploaded to the server.",
        file_to_send
    ))
}
/**
 * Function to execute "download" commands, file id and save path needs to be provided to the download command
 */
async fn execute_download_command(
    uuid: Uuid,
    cmd: Vec<&str>,
    download_url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("Executing download command: {}", cmd.join(" "));

    if cmd.len() != 3 {
        println!("Could not execute download command.");
        return Err(format!("Wrong number of parameters provided").into());
    }
    let file_id = cmd.get(1).unwrap();
    let save_path = cmd.get(2).unwrap();

    if let Err(e) = download::receive_file(uuid, file_id, Path::new(save_path), download_url).await
    {
        eprintln!("Error receiving file: {}", e);
        return Err(e);
    }
    Ok(format!(
        "File with name '{}' was saved to '{}'.",
        file_id, save_path
    ))
}

fn execute_kill_command(cmd: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Implement kill logic here
    println!("Executing kill command: {}", cmd);
    // Example: Terminate a process or service
    Ok("some string".to_string())
}

fn execute_start_command(cmd: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Implement start logic here
    println!("Executing start command: {}", cmd);
    // Example: Start a process or service
    Ok("some string".to_string())
}

fn execute_restart_command(cmd: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Implement restart logic here
    println!("Executing restart command: {}", cmd);
    // Example: Restart a process or service
    Ok("some string".to_string())
}
