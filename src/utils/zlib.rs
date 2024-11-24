use flate2::read::ZlibDecoder;
use std::io::prelude::*;

pub fn uncompress(source_buffer: &[u8]) -> Option<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(source_buffer);
    let mut buffer = Vec::new();

    match decoder.read_to_end(&mut buffer) {
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}
