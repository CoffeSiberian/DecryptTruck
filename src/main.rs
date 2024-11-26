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

fn decrypt_bin_file(file_bin: Vec<u8>) -> Result<Vec<u8>, String> {
    let file_type = match try_read_u32(&file_bin) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    if file_type == SignatureType::PlainText as u32 {
        return Ok(file_bin);
    }

    if file_type == SignatureType::Encrypted as u32 {
        let mut data = match decrypt(&file_bin) {
            Ok(res) => res,
            Err(_) => return Err("Error decrypting data".to_string()),
        };

        match uncompress(&data.data) {
            Ok(res) => data.data = res,
            Err(e) => return Err(e),
        };

        let file_type_verify = match try_read_u32(&data.data) {
            Ok(file_type) => file_type,
            Err(e) => return Err(e),
        };

        if file_type_verify == SignatureType::PlainText as u32 {
            return Ok(data.data);
        }

        match decode(&data.data) {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    } else if file_type == SignatureType::Binary as u32 {
        match decode(&file_bin) {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    } else {
        Err("Invalid file type".to_string())
    }
}

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    let args_paths: (String, String) = match args.len() {
        1 => {
            eprintln!("No parameters provided");
            return;
        }
        2 => (args[1].clone(), args[1].clone()),
        3 => (args[1].clone(), args[2].clone()),
        _ => {
            eprintln!("Too many parameters");
            return;
        }
    };

    let bin_file = match read_file_bin(&args_paths.0) {
        Some(res) => res,
        None => {
            eprintln!("Error reading file");
            return;
        }
    };

    match decrypt_bin_file(bin_file) {
        Ok(res) => {
            save_to_file(&args_paths.1, res);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    };

    println!("{:?} ms", start.elapsed().as_millis());
}
