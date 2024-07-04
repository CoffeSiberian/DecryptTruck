use libloading::{Library, Symbol};
use std::env;
use std::fs::{write, File};
use std::io::Read;
use std::os::raw::c_uint;

const DLL_NAME: &str = "SII_Decrypt.dll";

type GetMemoryFormatType = extern "C" fn(arr_val: *const u8, leng: c_uint) -> c_uint;
type DecodeMemoryType = extern "C" fn(
    arr_val: *const u8,
    leng: c_uint,
    out_buf_ptr: *const u8,
    out_buf_ptr_leng: *const c_uint,
) -> c_uint;

fn read_file_bin(path: &String) -> Option<Vec<u8>> {
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

fn save_file(path: &String, file_data: &String) -> bool {
    match write(path, file_data) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn get_memory_format(bin_file: &Vec<u8>, dir_bin: &String) -> Option<u32> {
    let lib = match unsafe { Library::new(format!("{}/{}", dir_bin, DLL_NAME)) } {
        Ok(res) => res,
        Err(_) => return None,
    };

    let ptr = bin_file.as_ptr();

    let func: Symbol<GetMemoryFormatType> = unsafe {
        match lib.get(b"GetMemoryFormat") {
            Ok(res) => res,
            Err(_) => return None,
        }
    };

    let response = func(ptr, bin_file.len() as u32);
    return Some(response);
}

fn descript_mem_file(bin_file: &Vec<u8>, dir_bin: &String) -> Option<String> {
    let lib = match unsafe { Library::new(format!("{}/{}", dir_bin, DLL_NAME)) } {
        Ok(res) => res,
        Err(_) => return None,
    };

    let ptr = bin_file.as_ptr();

    let func: Symbol<DecodeMemoryType> = unsafe {
        match lib.get(b"DecryptAndDecodeMemory") {
            Ok(res) => res,
            Err(_) => return None,
        }
    };

    let out_buf_size: u32 = 0;
    let out_buf_ptr = &out_buf_size as *const c_uint;

    let response = func(ptr, bin_file.len() as c_uint, 0 as *const u8, out_buf_ptr);

    if response != 0 {
        return None;
    }

    let new_file_data: Vec<u8> = vec![0; out_buf_size as usize];
    let new_file_data_ptr = new_file_data.as_ptr();

    func(
        ptr,
        bin_file.len() as c_uint,
        new_file_data_ptr,
        out_buf_ptr,
    );

    let to_string = match String::from_utf8(new_file_data) {
        Ok(res) => res,
        Err(_) => return None,
    };

    return Some(to_string);
}

fn descript_3nk_file(bin_file: &Vec<u8>, dir_bin: &String) -> Option<String> {
    let lib = match unsafe { Library::new(format!("{}/{}", dir_bin, DLL_NAME)) } {
        Ok(res) => res,
        Err(_) => return None,
    };

    let ptr = bin_file.as_ptr();

    let func: Symbol<DecodeMemoryType> = unsafe {
        match lib.get(b"DecodeMemory") {
            Ok(res) => res,
            Err(_) => return None,
        }
    };

    let out_buf_size: u32 = 0;
    let out_buf_ptr = &out_buf_size as *const c_uint;

    let response = func(ptr, bin_file.len() as c_uint, 0 as *const u8, out_buf_ptr);

    if response != 0 {
        return None;
    }

    let new_file_data: Vec<u8> = vec![0; out_buf_size as usize];
    let new_file_data_ptr = new_file_data.as_ptr();

    func(
        ptr,
        bin_file.len() as c_uint,
        new_file_data_ptr,
        out_buf_ptr,
    );

    let to_string = match String::from_utf8(new_file_data) {
        Ok(res) => res,
        Err(_) => return None,
    };

    return Some(to_string);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let args_paths: (String, String) = match args.len() {
        1 => {
            println!("No file path provided");
            return;
        }
        2 => (args[1].clone(), args[1].clone()),
        3 => (args[1].clone(), args[2].clone()),
        _ => {
            println!("Too many arguments");
            return;
        }
    };

    let dir_bin = match env::current_exe() {
        Ok(dir) => match dir.parent() {
            Some(res) => res.to_str().unwrap().to_string(),
            None => {
                println!("Error getting current directory");
                return;
            }
        },

        Err(_) => {
            println!("Error getting current directory");
            return;
        }
    };

    let bin_file = match read_file_bin(&args_paths.0) {
        Some(res) => res,
        None => {
            println!("Error reading file");
            return;
        }
    };

    let memory_format = match get_memory_format(&bin_file, &dir_bin) {
        Some(res) => res,
        None => {
            println!("Error getting memory format");
            return;
        }
    };

    match memory_format {
        1 => {
            match String::from_utf8(bin_file) {
                Ok(res) => save_file(&args_paths.1, &res),
                Err(_) => {
                    println!("Error converting file to string");
                    return;
                }
            };
        }
        2 => {
            match descript_mem_file(&bin_file, &dir_bin) {
                Some(res) => save_file(&args_paths.1, &res),
                None => {
                    println!("Error decrypting file");
                    return;
                }
            };
        }
        4 => {
            match descript_3nk_file(&bin_file, &dir_bin) {
                Some(res) => save_file(&args_paths.1, &res),
                None => {
                    println!("Error decrypting file");
                    return;
                }
            };
        }
        _ => {
            println!("Memory format not supported");
        }
    }
}
