use crate::sysinfo::saver::SysInfo;
use std::fs::File;
use std::io::Read;
use uuid::Uuid;

// Check and retrieve UUID from a file, or generate a new one if the file does not exist
pub fn check_uuid(filename: &str) -> Uuid {
    match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Err(_) = file.read_to_string(&mut contents) {
                return Uuid::new_v4();
            }
            if let Ok(system_info) = serde_json::from_str::<SysInfo>(&contents) {
                return system_info.uuid;
            }
            Uuid::new_v4()
        }
        Err(_) => Uuid::new_v4(),
    }
}
