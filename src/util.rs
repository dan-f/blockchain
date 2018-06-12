pub fn to_hex_string(hash: &[u8]) -> String {
    let strs: Vec<String> = hash.iter().map(|byte| format!("{:x?}", byte)).collect();
    strs.join("")
}
