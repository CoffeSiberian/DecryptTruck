mod decoder;
mod strucs;
mod utils;

use decoder::bsii_decoder::decode;

use strucs::data_sii::SignatureType;
use utils::aes::decrypt;
use utils::file_type::try_read_u32;
use utils::zlib::uncompress;

pub use utils::file_operations::{read_file_bin, save_to_file};

pub fn decrypt_bin_file(file_bin: &Vec<u8>) -> Result<Vec<u8>, String> {
    let file_type = match try_read_u32(&file_bin) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    if file_type == SignatureType::PlainText as u32 {
        return Ok(file_bin.clone());
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
