use flate2::read::ZlibDecoder;
use std::io::prelude::*;

pub fn uncompress(source_buffer: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder = ZlibDecoder::new(source_buffer);
    let mut buffer = Vec::with_capacity(source_buffer.len());

    match decoder.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(_) => Err("Error uncompressing data".to_string()),
    }
}
