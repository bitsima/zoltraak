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
) -> Result<(), String> {
    let cmd_parts = cmd.split_ascii_whitespace().collect::<Vec<&str>>();
    let cmd_type = cmd_parts.first().unwrap_or(&"");

    match *cmd_type {
        "shell" => execute_shell_command(cmd_parts)?,
        // upload <file_name>(optional)
        "upload" => execute_upload_command(uuid, cmd_parts, upload_url).await?,
        // download <file_id> <file_save_path>
        "download" => execute_download_command(uuid, cmd_parts, download_url).await?,
        // "kill" => execute_kill_command(cmd_parts),
        // "start" => execute_start_command(cmd_parts),
        // "restart" => execute_restart_command(cmd_parts),
        _ => {
            println!("Unsupported command type: {}", cmd_type);
            return Err(format!("Unsupported command type: {}", cmd_type))?;
        }
    }
    Ok(())
}

fn execute_shell_command(cmd_parts: Vec<&str>) -> Result<(), String> {
    if cmd_parts.len() <= 1 {
        println!("Could not execute shell command.");
        return Err(format!("Wrong number of parameters provided"));
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
        Ok(())
    } else {
        let error_str = String::from_utf8_lossy(&output.stderr);
        println!("Command failed with error: {}", error_str);
        Err(error_str.to_string())
    }
}

/**
 * Function to execute "upload" commands, will upload the default sysinfo file if the file name is not provided
 */
async fn execute_upload_command(uuid: Uuid, cmd: Vec<&str>, file_url: &str) -> Result<(), String> {
    println!("Executing upload command: {}", cmd.join(" "));
    if cmd.len() == 1 {
        // default file is sent
        if let Err(e) = upload::send_file(uuid, Path::new(DEFAULT_SYSINFO_FILE), file_url).await {
            eprintln!("Error sending file: {}", e);
            return Err(e.to_string());
        }
    } else if cmd.len() == 2 {
        // given file is sent
        if let Err(e) = upload::send_file(uuid, Path::new(cmd.get(1).unwrap()), file_url).await {
            eprintln!("Error sending file: {}", e);
            return Err(e.to_string());
        }
    } else {
        println!("Could not execute upload command.");
        return Err(format!("Wrong number of parameters provided"));
    }
    Ok(())
}
/**
 * Function to execute "download" commands, file id and save path needs to be provided to the download command
 */
async fn execute_download_command(
    uuid: Uuid,
    cmd: Vec<&str>,
    download_url: &str,
) -> Result<(), String> {
    println!("Executing download command: {}", cmd.join(" "));

    if cmd.len() != 3 {
        println!("Could not execute download command.");
        return Err(format!("Wrong number of parameters provided"));
    }
    let file_id = cmd.get(1).unwrap();
    let save_path = cmd.get(2).unwrap();

    if let Err(e) = download::receive_file(uuid, file_id, Path::new(save_path), download_url).await {
        eprintln!("Error receiving file: {}", e);
        return Err(e.to_string());
    }
    Ok(())
}

fn execute_kill_command(cmd: &str) -> Result<(), String> {
    // Implement kill logic here
    println!("Executing kill command: {}", cmd);
    // Example: Terminate a process or service
    Ok(())
}

fn execute_start_command(cmd: &str) -> Result<(), String> {
    // Implement start logic here
    println!("Executing start command: {}", cmd);
    // Example: Start a process or service
    Ok(())
}

fn execute_restart_command(cmd: &str) -> Result<(), String> {
    // Implement restart logic here
    println!("Executing restart command: {}", cmd);
    // Example: Restart a process or service
    Ok(())
}
