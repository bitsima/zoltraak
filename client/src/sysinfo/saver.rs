use crate::sysinfo::uuid::check_uuid;
use interfaces::Interface;
use mac_address::get_mac_address;
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Write};
use sysinfo::System;
use uuid::Uuid;

const FILENAME: &str = "sysinfo.dat";

pub fn save_file() -> Result<(Uuid, String), Box<dyn std::error::Error>> {
    let sys_info = gather_system_info()?;
    save_info_to_file(&sys_info)?;
    Ok((sys_info.uuid, sys_info.mac_address))
}

fn gather_system_info() -> Result<SysInfo, Box<dyn std::error::Error>> {
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

    let mac_address_opt = get_mac_address().expect("Couldn't get mac address");
    let mac_address = mac_address_opt
        .map(|addr| addr.to_string())
        .unwrap_or_else(|| "No MAC address found".to_string());

    let processes_list = sys
        .processes()
        .iter()
        .map(|(_, process)| process.name().to_string())
        .collect();

    Ok(SysInfo {
        uuid: check_uuid(FILENAME),
        mac_address: mac_address.clone(),
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
        running_processes: processes_list,
    })
}

fn save_info_to_file(info: &SysInfo) -> Result<(), Box<dyn std::error::Error>> {
    let info_json = serde_json::to_string(info)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILENAME)?;
    file.write_all(info_json.as_bytes())?;
    Ok(())
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
pub struct SysInfo {
    pub uuid: Uuid,
    mac_address: String,
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
    running_processes: Vec<String>,
}
