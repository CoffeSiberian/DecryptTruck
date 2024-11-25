mod decoder;
mod strucs;
mod utils;

use decoder::bsii_decoder::decode;
use std::env;
use std::io::Write;
use std::time::Instant;
use std::{fs::File, io::Read};
use strucs::data_sii::SignatureType;
use utils::aes::decrypt;
use utils::file_type::try_read_u32;
use utils::zlib::uncompress;

fn save_to_file(filename: &str, data: Vec<u8>) -> Option<()> {
    let mut file = match File::create(filename) {
        Ok(res) => res,
        Err(_) => return None,
    };

    match file.write_all(&data) {
        Ok(_) => Some(()),
        Err(_) => None,
    }
}

fn read_file_bin(path: &str) -> Option<Vec<u8>> {
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

fn decrypt_bin_file(file_bin: Vec<u8>) -> Option<Vec<u8>> {
    let file_type = match try_read_u32(&file_bin) {
        Some(file_type) => file_type,
        None => return None,
    };

    if file_type == SignatureType::PlainText as u32 {
        return Some(file_bin);
    }

    if file_type == SignatureType::Encrypted as u32 {
        let mut data = match decrypt(&file_bin) {
            Some(res) => res,
            None => return None,
        };

        match uncompress(&data.data) {
            Some(res) => data.data = res,
            None => return None,
        };

        let file_type_verify = match try_read_u32(&data.data) {
            Some(file_type) => file_type,
            None => return None,
        };

        if file_type_verify == SignatureType::PlainText as u32 {
            return Some(data.data);
        }

        match decode(&data.data) {
            Some(res) => Some(res),
            None => None,
        }
    } else if file_type == SignatureType::Binary as u32 {
        match uncompress(&file_bin) {
            Some(res) => Some(res),
            None => None,
        }
    } else {
        None
    }
}

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    let args_paths: (String, String) = match args.len() {
        1 => panic!("Error reading file"),
        2 => (args[1].clone(), args[1].clone()),
        3 => (args[1].clone(), args[2].clone()),
        _ => panic!("Error reading file"),
    };

    let bin_file = match read_file_bin(&args_paths.0) {
        Some(res) => res,
        None => panic!("Error reading file"),
    };

    match decrypt_bin_file(bin_file) {
        Some(res) => {
            save_to_file(&args_paths.1, res);
        }
        None => println!("Error decrypting file"),
    };

    println!("{:?} ms", start.elapsed().as_millis());
}
