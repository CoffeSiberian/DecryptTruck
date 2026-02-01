use decrypt_truck::decrypt_bin_file;
use std::{fs::File, io::Read};

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

#[test]
fn test_decrypt_bin_encrypted_file() {
    let file_bin = read_file_bin("tests/test_data/encrypted_sig/game.sii").unwrap();
    let decrypted_file = decrypt_bin_file(&file_bin).unwrap();

    let file_compare = read_file_bin("tests/test_data/plain_text_sig/game.sii").unwrap();

    assert!(
        decrypted_file == file_compare,
        "Decrypted file is not the same as the original file"
    );
}

#[test]
fn test_decrypt_bin_encrypted_file_with_mod() {
    let file_bin = read_file_bin("tests/test_data/encrypted_sig/game_with_mod.sii").unwrap();
    let decrypted_file = decrypt_bin_file(&file_bin).unwrap();

    let file_compare = read_file_bin("tests/test_data/plain_text_sig/game_with_mod.sii").unwrap();

    assert!(
        decrypted_file == file_compare,
        "Decrypted file with mod is not the same as the original file"
    );
}

#[test]
fn test_decrypt_bin_encrypted_file_with_mod_2() {
    let file_bin = read_file_bin("tests/test_data/encrypted_sig/game_with_mod_2.sii").unwrap();
    let decrypted_file = decrypt_bin_file(&file_bin).unwrap();

    let file_compare = read_file_bin("tests/test_data/plain_text_sig/game_with_mod_2.sii").unwrap();

    assert!(
        decrypted_file == file_compare,
        "Decrypted file with mod is not the same as the original file"
    );
}
