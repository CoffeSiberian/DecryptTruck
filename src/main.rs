mod decoder;
mod strucs;
mod utils;

use decoder::bsii_decoder::decode;
use std::env;
use std::io::Write;
use std::time::Instant;
use std::{fs::File, io::Read};
use utils::aes::decrypt;
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
    let mut data = match decrypt(&file_bin) {
        Some(res) => res,
        None => return None,
    };

    match uncompress(&data.data) {
        Some(res) => data.data = res,
        None => return None,
    };

    match decode(&data.data) {
        Some(res) => Some(res),
        None => None,
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
