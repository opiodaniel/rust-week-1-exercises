// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {

    // Make sure the hex string is at least 8 characters (4 bytes)
    if raw_tx_hex.len() < 8 {
        return Err("Hex string too short to contain version".to_string());
    }

    // Take the first 8 characters which represent 4 bytes
    let version_hex = &raw_tx_hex[0..8];

    // Convert each 2 hex chars to a byte and collect into a [u8; 4] array
    let mut bytes = [0u8; 4];
    for i in 0..4 {
        let start_index = i * 2;
        let byte_str = &version_hex[start_index..start_index + 2];
        bytes[i] = u8::from_str_radix(byte_str, 16)
            .map_err(|e| format!("Failed to parse byte {}: {}", i, e))?;
    }

    // Convert from little-endian byte array to u32
    let version = u32::from_le_bytes(bytes);
    Ok(version)

}

