use std::time::{SystemTime, UNIX_EPOCH};

pub fn to_hex_string(hash: &[u8]) -> String {
    let strs: Vec<String> = hash.iter().map(|byte| format!("{:x?}", byte)).collect();
    strs.join("")
}

pub fn millis_since_unix_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Somehow the current time is before the unix epoch")
        .as_secs() * 1000
}
