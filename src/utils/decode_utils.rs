use crate::strucs::data_sii::IDComplexType;
use crate::strucs::float_vector::{
    Int32Vector2, Int32Vector3i32, SingleVector2, SingleVector3, SingleVector4, SingleVector7,
    SingleVector8,
};
use std::collections::HashMap;
use std::str;

const CHAR_TABLE: &'static [char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '_',
];

// 0x01
pub fn decode_utf8_string(bytes: &[u8], offset: &mut usize) -> String {
    let length = decode_u32(bytes, offset) as usize;
    let bytes = str::from_utf8(&bytes[*offset..*offset + length]);

    let result = match bytes {
        Ok(res) => res.to_string(),
        Err(_) => panic!("Error decoding utf8 string"),
    };

    *offset += length;
    result
}

// 0x02
pub fn decode_utf8_string_array(bytes: &[u8], offset: &mut usize) -> Vec<String> {
    let number_of_strings = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number_of_strings as usize);

    for _ in 0..number_of_strings {
        result.push(decode_utf8_string(bytes, offset));
    }

    result
}

// 0x03
pub fn decode_u64_string(bytes: &[u8], offset: &mut usize) -> String {
    let mut result = String::new();
    let mut value = decode_u64(bytes, offset);

    // value &= !(1 << 63);
    while value != 0 {
        let mut char_idx = (value % 38) as isize;
        if char_idx < 0 {
            char_idx = -char_idx;
        }
        char_idx -= 1;
        value /= 38;
        if char_idx >= 0 && char_idx < 38 {
            result.push(CHAR_TABLE[char_idx as usize]);
        }
    }

    result
}

// 0x04
pub fn decode_u64_string_array(bytes: &[u8], offset: &mut usize) -> Vec<String> {
    let number_of_strings = decode_u32(bytes, offset);
    let mut result = vec![String::new(); number_of_strings as usize];

    for i in 0..number_of_strings {
        result[i as usize] = decode_u64_string(bytes, offset);
    }

    result
}

// 0x05
pub fn decode_single(bytes: &[u8], offset: &mut usize) -> f32 {
    let bytes = match bytes[*offset..*offset + std::mem::size_of::<f32>()].try_into() {
        Ok(res) => res,
        Err(_) => panic!("Error decoding single"),
    };

    let result = f32::from_le_bytes(bytes);
    *offset += std::mem::size_of::<f32>();

    result
}

// 0x06
pub fn decode_single_array(bytes: &[u8], offset: &mut usize) -> Vec<f32> {
    let number_of_singles = decode_u32(bytes, offset) as usize;
    let mut result = Vec::with_capacity(number_of_singles);

    for _ in 0..number_of_singles {
        result.push(decode_single(bytes, offset));
    }

    result
}

// 0x07
pub fn decode_single_vector2(bytes: &[u8], offset: &mut usize) -> SingleVector2 {
    SingleVector2 {
        a: decode_single(bytes, offset),
        b: decode_single(bytes, offset),
    }
}

// 0x08
pub fn decode_single_vector2_array(bytes: &[u8], offset: &mut usize) -> Vec<SingleVector2> {
    let number_of_vector2s = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number_of_vector2s as usize);

    for _ in 0..number_of_vector2s {
        result.push(decode_single_vector2(bytes, offset));
    }

    result
}

// 0x09
pub fn decode_single_vector3(bytes: &[u8], offset: &mut usize) -> SingleVector3 {
    SingleVector3 {
        a: decode_single(bytes, offset),
        b: decode_single(bytes, offset),
        c: decode_single(bytes, offset),
    }
}

// 0x0A
pub fn decode_single_vector3_array(bytes: &[u8], offset: &mut usize) -> Vec<SingleVector3> {
    let number_of_vector3s = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number_of_vector3s as usize);

    for _ in 0..number_of_vector3s {
        result.push(decode_single_vector3(bytes, offset));
    }

    result
}

// 0x11
pub fn decode_int32_vector3(bytes: &[u8], offset: &mut usize) -> Int32Vector3i32 {
    Int32Vector3i32 {
        a: decode_int32(bytes, offset),
        b: decode_int32(bytes, offset),
        c: decode_int32(bytes, offset),
    }
}

// 0x12
pub fn decode_int32_vector3_array(bytes: &[u8], offset: &mut usize) -> Vec<Int32Vector3i32> {
    let number_of_vector3s = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number_of_vector3s as usize);

    for _ in 0..number_of_vector3s {
        result.push(decode_int32_vector3(bytes, offset));
    }

    result
}

// 0x1A
pub fn decode_single_vector7_array(bytes: &[u8], offset: &mut usize) -> Vec<SingleVector7> {
    let number_of_vector7s = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number_of_vector7s as usize);

    for _ in 0..number_of_vector7s {
        result.push(decode_single_vector7(bytes, offset));
    }

    result
}

pub fn decode_single_vector8_array(bytes: &[u8], offset: &mut usize) -> Vec<SingleVector8> {
    let number_of_vector8s = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number_of_vector8s as usize);

    for _ in 0..number_of_vector8s {
        result.push(decode_single_vector8(bytes, offset));
    }

    result
}

pub fn decode_single_vector8(bytes: &[u8], offset: &mut usize) -> SingleVector8 {
    let mut result = SingleVector8 {
        a: decode_single(bytes, offset),
        b: decode_single(bytes, offset),
        c: decode_single(bytes, offset),
        d: decode_single(bytes, offset),
        e: decode_single(bytes, offset),
        f: decode_single(bytes, offset),
        g: decode_single(bytes, offset),
        h: decode_single(bytes, offset),
    };

    let bias = result.d as i64;

    let mut bits = bias;
    bits &= 0xFFF;
    bits -= 2048;
    bits <<= 9;
    result.a += bits as f32;

    let mut bits2 = bias >> 12;
    bits2 &= 0xFFF;
    bits2 -= 2048;
    bits2 <<= 9;
    result.c += bits2 as f32;

    result
}

// 0x17
pub fn decode_single_vector4(bytes: &[u8], offset: &mut usize) -> SingleVector4 {
    SingleVector4 {
        a: decode_single(bytes, offset),
        b: decode_single(bytes, offset),
        c: decode_single(bytes, offset),
        d: decode_single(bytes, offset),
    }
}

// 0x18
pub fn decode_single_vector4_array(bytes: &[u8], offset: &mut usize) -> Vec<SingleVector4> {
    let number = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number as usize);

    for _ in 0..number {
        result.push(decode_single_vector4(bytes, offset));
    }

    result
}

// 0x19
pub fn decode_single_vector7(bytes: &[u8], offset: &mut usize) -> SingleVector7 {
    SingleVector7 {
        a: decode_single(bytes, offset),
        b: decode_single(bytes, offset),
        c: decode_single(bytes, offset),
        d: decode_single(bytes, offset),
        e: decode_single(bytes, offset),
        f: decode_single(bytes, offset),
        g: decode_single(bytes, offset),
    }
}

// 0x25
pub fn decode_int32(bytes: &[u8], offset: &mut usize) -> i32 {
    let bytes = match bytes[*offset..*offset + std::mem::size_of::<i32>()].try_into() {
        Ok(res) => res,
        Err(_) => panic!("Error decoding int32"),
    };

    let result = i32::from_le_bytes(bytes);
    *offset += std::mem::size_of::<i32>();

    result
}

// 0x26
pub fn decode_i32_array(bytes: &[u8], offset: &mut usize) -> Vec<i32> {
    let number_of_ints = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number_of_ints as usize);

    for _ in 0..number_of_ints {
        result.push(decode_int32(bytes, offset));
    }

    result
}

// 0x2B
pub fn decode_u16(bytes: &[u8], offset: &mut usize) -> u16 {
    let bytes = match bytes[*offset..*offset + std::mem::size_of::<u16>()].try_into() {
        Ok(res) => res,
        Err(_) => panic!("Error decoding u16"),
    };

    let result = u16::from_le_bytes(bytes);
    *offset += std::mem::size_of::<u16>();

    result
}

// 0x2C
pub fn decode_u16_array(bytes: &[u8], offset: &mut usize) -> Vec<u16> {
    let number_of_ints = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number_of_ints as usize);

    for _ in 0..number_of_ints {
        result.push(decode_u16(bytes, offset));
    }

    result
}

// 0x27 and 0x2F
pub fn decode_u32(bytes: &[u8], offset: &mut usize) -> u32 {
    let bytes = match bytes[*offset..*offset + 4].try_into() {
        Ok(res) => res,
        Err(_) => panic!("Error decoding u32"),
    };

    let result = u32::from_le_bytes(bytes);
    *offset += std::mem::size_of::<u32>();

    result
}

// 0x28
pub fn decode_u32_array(bytes: &[u8], offset: &mut usize) -> Vec<u32> {
    let number_of_ints = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number_of_ints as usize);

    for _ in 0..number_of_ints {
        result.push(decode_u32(bytes, offset));
    }

    result
}

// 0x29
pub fn decode_int16(bytes: &[u8], offset: &mut usize) -> i16 {
    let bytes = match bytes[*offset..*offset + std::mem::size_of::<i16>()].try_into() {
        Ok(res) => res,
        Err(_) => panic!("Error decoding int16"),
    };

    let result = i16::from_le_bytes(bytes);
    *offset += std::mem::size_of::<i16>();

    result
}

// 0x2A
pub fn decode_int16_array(bytes: &[u8], offset: &mut usize) -> Vec<i16> {
    let number_of_ints = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number_of_ints as usize);

    for _ in 0..number_of_ints {
        result.push(decode_int16(bytes, offset));
    }

    result
}

// 0x31
pub fn decode_int64(bytes: &[u8], offset: &mut usize) -> i64 {
    let bytes = match bytes[*offset..*offset + std::mem::size_of::<i64>()].try_into() {
        Ok(res) => res,
        Err(_) => panic!("Error decoding int64"),
    };

    let result = i64::from_le_bytes(bytes);
    *offset += std::mem::size_of::<i64>();

    result
}

// 0x32
pub fn decode_int64_array(bytes: &[u8], offset: &mut usize) -> Vec<i64> {
    let number_of_ints = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number_of_ints as usize);

    for _ in 0..number_of_ints {
        result.push(decode_int64(bytes, offset));
    }

    result
}

// 0x33
pub fn decode_u64(bytes: &[u8], offset: &mut usize) -> u64 {
    let bytes = match bytes[*offset..*offset + 8].try_into() {
        Ok(res) => res,
        Err(_) => panic!("Error decoding u64"),
    };

    let result = u64::from_le_bytes(bytes);
    *offset += std::mem::size_of::<u64>();

    result
}

// 0x34
pub fn decode_u64_array(bytes: &[u8], offset: &mut usize) -> Vec<u64> {
    let number_of_ints = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number_of_ints as usize);

    for _ in 0..number_of_ints {
        result.push(decode_u64(bytes, offset));
    }

    result
}

// 0x35
pub fn decode_bool(bytes: &[u8], offset: &mut usize) -> bool {
    let result = bytes[*offset] != 0;
    *offset += std::mem::size_of::<bool>();
    result
}

// 0x36
pub fn decode_bool_array(bytes: &[u8], offset: &mut usize) -> Vec<bool> {
    let number_of_bools = decode_u32(bytes, offset);
    let mut result = vec![false; number_of_bools as usize];

    for i in 0..number_of_bools {
        result[i as usize] = decode_bool(bytes, offset);
    }

    result
}

//0x37
pub fn decode_ordinal_string_list(bytes: &[u8], offset: &mut usize) -> HashMap<u32, String> {
    let length = decode_u32(bytes, offset);
    let mut values = HashMap::new();

    for _ in 0..length {
        let ordinal = decode_u32(bytes, offset);
        let string_value = decode_utf8_string(bytes, offset);
        values.insert(ordinal, string_value);
    }

    values
}

pub fn get_ordinal_string_from_values(
    values: &std::collections::HashMap<u32, String>,
    bytes: &[u8],
    offset: &mut usize,
) -> String {
    let index = decode_u32(bytes, offset);

    if let Some(value) = values.get(&index) {
        value.clone()
    } else {
        String::new()
    }
}

// 0x39, 0x3B, 0x3D
pub fn decode_id(bytes: &[u8], offset: &mut usize) -> IDComplexType {
    let mut result = IDComplexType::new();

    result.value = String::new();
    result.part_count = bytes[*offset];
    *offset += 1;

    if result.part_count == 0xFF {
        result.address = decode_u64(bytes, offset);

        let data = result.address.to_le_bytes();
        let mut parts = vec![String::new(); data.len() / 2];
        let mut current_part = String::new();

        for (i, &byte) in data.iter().enumerate() {
            if i % 2 == 0 && i > 0 {
                if i >= data.len() - 2 {
                    while current_part.starts_with('0') {
                        current_part.remove(0);
                    }
                }

                if !current_part.is_empty() {
                    result.value = format!("{}.{}", current_part, result.value);
                }

                parts[(data.len() / 2) - (i / 2)] = current_part.clone();
                current_part.clear();
            }

            current_part = format!("{:02x}{}", byte, current_part);

            if i == data.len() - 1 {
                while current_part.starts_with('0') {
                    current_part.remove(0);
                }

                if !current_part.is_empty() {
                    result.value = format!("{}.{}", current_part, result.value);
                }

                parts[0] = current_part.clone();
                current_part.clear();
            }
        }

        result.value = format!("_nameless.{}", &result.value[..result.value.len() - 1]);
    } else {
        for i in 0..result.part_count {
            let s = decode_u64_string(bytes, offset);

            if i > 0 {
                result.value.push('.');
            }

            result.value.push_str(&s);
        }

        if result.part_count == 0 {
            result.value = "null".to_string();
        }
    }

    result
}

// 0x3A, 0x3C, 0x3E
pub fn decode_id_array(bytes: &[u8], offset: &mut usize) -> Vec<IDComplexType> {
    let number_of_ids = decode_u32(bytes, offset);
    let mut result = Vec::with_capacity(number_of_ids as usize);

    for _ in 0..number_of_ids {
        result.push(decode_id(bytes, offset));
    }

    result
}

// 0x41
pub fn decode_int32_vector2(bytes: &[u8], offset: &mut usize) -> Int32Vector2 {
    Int32Vector2 {
        a: decode_int32(bytes, offset),
        b: decode_int32(bytes, offset),
    }
}
