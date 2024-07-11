use std::process::Command;

pub fn execute_command(cmd: &str) -> Result<(), String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .map_err(|e| format!("Failed to execute process: {}", e))?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("Command output: {}", output_str);
    } else {
        let error_str = String::from_utf8_lossy(&output.stderr);
        println!("Command failed with error: {}", error_str);
    }

    Ok(())
}
