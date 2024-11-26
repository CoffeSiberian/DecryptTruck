pub fn try_read_u32(bytes: &[u8]) -> Result<u32, String> {
    if bytes.len() < std::mem::size_of::<u32>() {
        return Err("Invalid file size".to_string());
    }

    match bytes[0..4].try_into() {
        Ok(array) => return Ok(u32::from_le_bytes(array)),
        Err(_) => return Err("Invalid file size".to_string()),
    };
}
