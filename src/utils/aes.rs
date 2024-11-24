use crate::strucs::data_sii::{SIIData, SIIHeader};
use aes::Aes256;
use cipher::block_padding::Pkcs7;
use cipher::{BlockDecryptMut, KeyIvInit};
use std::convert::TryInto;

type Aes256CbcDec = cbc::Decryptor<Aes256>;

pub const SII_KEY: [u8; 32] = [
    0x2a, 0x5f, 0xcb, 0x17, 0x91, 0xd2, 0x2f, 0xb6, 0x02, 0x45, 0xb3, 0xd8, 0x36, 0x9e, 0xd0, 0xb2,
    0xc2, 0x73, 0x71, 0x56, 0x3f, 0xbf, 0x1f, 0x3c, 0x9e, 0xdf, 0x6b, 0x11, 0x82, 0x5a, 0x5d, 0x0a,
];

pub fn decrypt(encrypted: &Vec<u8>) -> Option<SIIData> {
    let mut header = SIIHeader::new();

    // let mut hmac: Vec<u8> = Vec::new();
    let mut iv: Vec<u8> = Vec::new();

    let mut stream_pos = 0;

    if encrypted.len() - stream_pos >= std::mem::size_of::<u32>() {
        let bytes = match encrypted[stream_pos..stream_pos + std::mem::size_of::<u32>()].try_into()
        {
            Ok(res) => res,
            Err(_) => return None,
        };

        header.signature = u32::from_le_bytes(bytes);
        stream_pos += std::mem::size_of::<u32>();
    }

    if encrypted.len() - stream_pos >= 32 {
        // hmac = encrypted[stream_pos..stream_pos + 32].to_vec();
        stream_pos += 32;
    }

    if encrypted.len() - stream_pos >= 16 {
        iv = encrypted[stream_pos..stream_pos + 16].to_vec();
        stream_pos += 16;
    }

    if encrypted.len() - stream_pos >= std::mem::size_of::<u32>() {
        let bytes = match encrypted[stream_pos..stream_pos + std::mem::size_of::<u32>()].try_into()
        {
            Ok(res) => res,
            Err(_) => return None,
        };

        header.data_size = u32::from_le_bytes(bytes);
        stream_pos += std::mem::size_of::<u32>();
    }

    let final_encrypted = encrypted[stream_pos..].to_vec();

    // Datos cifrados
    let cipher = match Aes256CbcDec::new_from_slices(&SII_KEY, &iv) {
        Ok(res) => res,
        Err(_) => return None,
    };

    let mut final_encrypted_mut = final_encrypted;

    let decrypted_data = match cipher.decrypt_padded_mut::<Pkcs7>(&mut final_encrypted_mut) {
        Ok(decrypted_data) => decrypted_data,
        Err(_) => return None,
    };

    Some(SIIData {
        data: decrypted_data.to_vec(),
        header,
    })
}
