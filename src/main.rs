use decrypt_truck::decrypt_bin_file;
use std::{
    env,
    fs::File,
    io::{Read, Write},
    time::Instant,
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

    match decrypt_bin_file(&bin_file) {
        Ok(res) => {
            save_to_file(&args_paths.1, res);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    };

    println!("{:?} ms", start.elapsed().as_millis());
}
