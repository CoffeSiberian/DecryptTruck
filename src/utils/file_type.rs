pub fn try_read_u32(bytes: &[u8]) -> Option<u32> {
    if bytes.len() < std::mem::size_of::<u32>() {
        return None;
    }

    match bytes[0..4].try_into() {
        Ok(array) => return Some(u32::from_le_bytes(array)),
        Err(_) => return None,
    };
}
