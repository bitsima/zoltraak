use interfaces::Interface;
use serde::Serialize;
use std::{fs::File, io::Write};
use sysinfo::System;

#[derive(Serialize)]
struct NetworkAddress {
    kind: String,
    addr: Option<String>,
    mask: Option<String>,
    hop: Option<String>,
}

#[derive(Serialize)]
struct NetworkInterface {
    name: String,
    addresses: Vec<NetworkAddress>,
    flags: String,
}

#[derive(Serialize)]
struct SysInfo {
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

fn main() {
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

    let info = SysInfo {
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

    let json_data = serde_json::to_string(&info).expect("Failed to serialize data");

    let mut file = File::create("sysinfo.dat").expect("Unable to create file");
    file.write_all(json_data.as_bytes())
        .expect("Unable to write data");
}
