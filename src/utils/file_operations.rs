use std::{
    fs::File,
    io::{Read, Write},
};

pub fn save_to_file(filename: &str, data: Vec<u8>) -> Option<()> {
    let mut file = match File::create(filename) {
        Ok(res) => res,
        Err(_) => return None,
    };

    match file.write_all(&data) {
        Ok(_) => Some(()),
        Err(_) => None,
    }
}

pub fn read_file_bin(path: &str) -> Option<Vec<u8>> {
    let mut file = match File::open(path) {
        Ok(res) => res,
        Err(_) => return None,
    };

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}
