use decrypt_truck::{decrypt_bin_file, read_file_bin};

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
