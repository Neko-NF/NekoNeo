use sha2::{Digest, Sha256};
use sysinfo::System;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

pub fn compute_fingerprint() -> String {
    let mut hasher = Sha256::new();

    hasher.update(b"nekoneo-device-fingerprint-v1");

    let mut system = System::new_all();
    system.refresh_cpu_all();
    for cpu in system.cpus() {
        hasher.update(cpu.brand().as_bytes());
    }

    if let Some(hostname) = System::host_name() {
        hasher.update(hostname.as_bytes());
    }

    for (name, network) in sysinfo::Networks::new_with_refreshed_list().iter() {
        let mac = network.mac_address();
        if mac.0 == [0u8; 6] {
            continue;
        }
        hasher.update(name.as_bytes());
        hasher.update(&mac.0);
    }

    if let Some(guid) = read_machine_guid() {
        hasher.update(guid.as_bytes());
    }

    format!("{:x}", hasher.finalize())
}

fn read_machine_guid() -> Option<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Cryptography")
        .ok()?;
    key.get_value("MachineGuid").ok()
}
