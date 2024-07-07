use interfaces::Interface;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, Read, Write},
};
use sysinfo::System;
use uuid::Uuid;

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

#[derive(Clone, Serialize, Deserialize)]
struct UUID {
    uuid: Uuid,
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

    let uuid = match File::open("sysinfo.dat") {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Err(_) = file.read_to_string(&mut contents) {
                UUID {
                    uuid: Uuid::new_v4(),
                }
            } else if let Ok(uuid_part) = serde_json::from_str::<HashMap<String, UUID>>(&contents) {
                uuid_part.get("uuid").cloned().unwrap_or(UUID {
                    uuid: Uuid::new_v4(),
                })
            } else {
                UUID {
                    uuid: Uuid::new_v4(),
                }
            }
        }
        Err(_) => UUID {
            uuid: Uuid::new_v4(),
        },
    };

    let uuid_json = serde_json::to_string(&uuid).expect("Failed to serialize UUID");
    let info_json = serde_json::to_string(&info).expect("Failed to serialize SysInfo");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("sysinfo.dat")?;
    file.write_all(uuid_json.as_bytes())?;
    file.write_all(info_json.as_bytes())?;

    Ok(uuid.uuid)
}
