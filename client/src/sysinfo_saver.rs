use interfaces::Interface;
use mac_address::get_mac_address;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
};
use sysinfo::System;
use uuid::Uuid;

const FILENAME: &str = "sysinfo.dat";

#[derive(Serialize, Deserialize)]
struct NetworkAddress {
    kind: String,
    addr: Option<String>,
    mask: Option<String>,
    hop: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct NetworkInterface {
    name: String,
    addresses: Vec<NetworkAddress>,
    flags: String,
}

#[derive(Serialize, Deserialize)]
struct SysInfo {
    uuid: Uuid,
    mac_address_str: String,
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
    system_name: String,
    kernel_version: String,
    os_version: String,
    host_name: String,
    nb_cpus: usize,
    network_interfaces: Vec<NetworkInterface>,
}

fn convert_address(addr: &interfaces::Address) -> NetworkAddress {
    NetworkAddress {
        kind: format!("{:?}", addr.kind),
        addr: addr.addr.map(|a| a.to_string()),
        mask: addr.mask.map(|m| m.to_string()),
        hop: addr.hop.map(|h| h.to_string()),
    }
}

fn convert_flags(flags: interfaces::InterfaceFlags) -> String {
    format!("{:?}", flags)
}

fn check_uuid(filename: &str) -> Uuid {
    return match File::open(filename) {
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
    };
}

pub fn save_file() -> io::Result<Uuid> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let network_interfaces = Interface::get_all()
        .expect("Could not get interfaces")
        .iter()
        .map(|iface| NetworkInterface {
            name: iface.name.clone(),
            addresses: iface.addresses.iter().map(convert_address).collect(),
            flags: convert_flags(iface.flags),
        })
        .collect();

    let mac_address = get_mac_address().expect("Couldn't get mac address");
    let mac_address_str = mac_address
        .map(|addr| addr.to_string())
        .unwrap_or_else(|| "No MAC address found".to_string());

    let info = SysInfo {
        uuid: check_uuid(FILENAME),
        mac_address_str,
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
        system_name: System::name().unwrap_or_default(),
        kernel_version: System::kernel_version().unwrap_or_default(),
        os_version: System::os_version().unwrap_or_default(),
        host_name: System::host_name().unwrap_or_default(),
        nb_cpus: sys.cpus().len(),
        network_interfaces,
    };

    let info_json = serde_json::to_string(&info).expect("Failed to serialize SysInfo");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILENAME)?;
    file.write_all(info_json.as_bytes())?;

    Ok(info.uuid)
}
