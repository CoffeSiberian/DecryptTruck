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
pub fn decode_utf8_string(bytes: &[u8], offset: &mut usize) -> Result<String, String> {
    let length = match decode_u32(bytes, offset) {
        Ok(res) => res as usize,
        Err(err) => return Err(err),
    };
    let bytes = str::from_utf8(&bytes[*offset..*offset + length]);

    let result = match bytes {
        Ok(res) => res.to_string(),
        Err(_) => return Err(format!("Error decoding utf8 string offset: {}", offset)),
    };

    *offset += length;
    Ok(result)
}

// 0x02
pub fn decode_utf8_string_array(bytes: &[u8], offset: &mut usize) -> Result<Vec<String>, String> {
    let number_of_strings = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_strings as usize);

    for _ in 0..number_of_strings {
        let value = match decode_utf8_string(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

// 0x03
pub fn decode_u64_string(bytes: &[u8], offset: &mut usize) -> Result<String, String> {
    let mut result = String::new();
    let mut value = match decode_u64(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    // value &= !(1 << 63);
    while value != 0 {
        let mut char_idx = (value % 38) as isize;
        if char_idx < 0 {
            char_idx = -char_idx;
        }
        char_idx -= 1;
        value /= 38;
        if (0..38).contains(&char_idx) {
            result.push(CHAR_TABLE[char_idx as usize]);
        }
    }

    Ok(result)
}

// 0x04
pub fn decode_u64_string_array(bytes: &[u8], offset: &mut usize) -> Result<Vec<String>, String> {
    let number_of_strings = match decode_u32(bytes, offset) {
        Ok(res) => res as usize,
        Err(err) => return Err(err),
    };
    let mut result = vec![String::new(); number_of_strings];

    for i in 0..number_of_strings {
        let value = match decode_u64_string(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result[i] = value;
    }

    Ok(result)
}

// 0x05
pub fn decode_single(bytes: &[u8], offset: &mut usize) -> Result<f32, String> {
    let bytes = match bytes[*offset..*offset + std::mem::size_of::<f32>()].try_into() {
        Ok(res) => res,
        Err(_) => return Err(format!("Error decoding single offset: {}", offset)),
    };

    let result = f32::from_le_bytes(bytes);
    *offset += std::mem::size_of::<f32>();

    Ok(result)
}

// 0x06
pub fn decode_single_array(bytes: &[u8], offset: &mut usize) -> Result<Vec<f32>, String> {
    let number_of_singles = match decode_u32(bytes, offset) {
        Ok(res) => res as usize,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_singles);

    for _ in 0..number_of_singles {
        let value = match decode_single(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

// 0x07
pub fn decode_single_vector2(bytes: &[u8], offset: &mut usize) -> Result<SingleVector2, String> {
    let a = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let b = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    Ok(SingleVector2 { a, b })
}

// 0x08
pub fn decode_single_vector2_array(
    bytes: &[u8],
    offset: &mut usize,
) -> Result<Vec<SingleVector2>, String> {
    let number_of_vector2s = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_vector2s as usize);

    for _ in 0..number_of_vector2s {
        let value = match decode_single_vector2(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

// 0x09
pub fn decode_single_vector3(bytes: &[u8], offset: &mut usize) -> Result<SingleVector3, String> {
    let a = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let b = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let c = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    Ok(SingleVector3 { a, b, c })
}

// 0x0A
pub fn decode_single_vector3_array(
    bytes: &[u8],
    offset: &mut usize,
) -> Result<Vec<SingleVector3>, String> {
    let number_of_vector3s = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_vector3s as usize);

    for _ in 0..number_of_vector3s {
        let value = match decode_single_vector3(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

// 0x11
pub fn decode_int32_vector3(bytes: &[u8], offset: &mut usize) -> Result<Int32Vector3i32, String> {
    let a = match decode_int32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let b = match decode_int32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let c = match decode_int32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    Ok(Int32Vector3i32 { a, b, c })
}

// 0x12
pub fn decode_int32_vector3_array(
    bytes: &[u8],
    offset: &mut usize,
) -> Result<Vec<Int32Vector3i32>, String> {
    let number_of_vector3s = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_vector3s as usize);

    for _ in 0..number_of_vector3s {
        let value = match decode_int32_vector3(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

// 0x1A
pub fn decode_single_vector7_array(
    bytes: &[u8],
    offset: &mut usize,
) -> Result<Vec<SingleVector7>, String> {
    let number_of_vector7s = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_vector7s as usize);

    for _ in 0..number_of_vector7s {
        let value = match decode_single_vector7(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

pub fn decode_single_vector8_array(
    bytes: &[u8],
    offset: &mut usize,
) -> Result<Vec<SingleVector8>, String> {
    let number_of_vector8s = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_vector8s as usize);

    for _ in 0..number_of_vector8s {
        let value = match decode_single_vector8(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

pub fn decode_single_vector8(bytes: &[u8], offset: &mut usize) -> Result<SingleVector8, String> {
    let a = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let b = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let c = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let d = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let e = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let f = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let g = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let h = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    let mut result = SingleVector8 {
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
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

    Ok(result)
}

// 0x17
pub fn decode_single_vector4(bytes: &[u8], offset: &mut usize) -> Result<SingleVector4, String> {
    let a = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let b = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let c = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let d = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    Ok(SingleVector4 { a, b, c, d })
}

// 0x18
pub fn decode_single_vector4_array(
    bytes: &[u8],
    offset: &mut usize,
) -> Result<Vec<SingleVector4>, String> {
    let number = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number as usize);

    for _ in 0..number {
        let value = match decode_single_vector4(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

// 0x19
pub fn decode_single_vector7(bytes: &[u8], offset: &mut usize) -> Result<SingleVector7, String> {
    let a = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let b = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let c = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let d = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let e = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let f = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let g = match decode_single(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    Ok(SingleVector7 {
        a,
        b,
        c,
        d,
        e,
        f,
        g,
    })
}

// 0x25
pub fn decode_int32(bytes: &[u8], offset: &mut usize) -> Result<i32, String> {
    let bytes = match bytes[*offset..*offset + std::mem::size_of::<i32>()].try_into() {
        Ok(res) => res,
        Err(_) => return Err(format!("Error decoding int32 offset: {}", offset)),
    };

    let result = i32::from_le_bytes(bytes);
    *offset += std::mem::size_of::<i32>();

    Ok(result)
}

// 0x26
pub fn decode_i32_array(bytes: &[u8], offset: &mut usize) -> Result<Vec<i32>, String> {
    let number_of_ints = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_ints as usize);

    for _ in 0..number_of_ints {
        let value = match decode_int32(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

// 0x2B
pub fn decode_u16(bytes: &[u8], offset: &mut usize) -> Result<u16, String> {
    let bytes = match bytes[*offset..*offset + std::mem::size_of::<u16>()].try_into() {
        Ok(res) => res,
        Err(_) => return Err(format!("Error decoding u16 offset: {}", offset)),
    };

    let result = u16::from_le_bytes(bytes);
    *offset += std::mem::size_of::<u16>();

    Ok(result)
}

// 0x2C
pub fn decode_u16_array(bytes: &[u8], offset: &mut usize) -> Result<Vec<u16>, String> {
    let number_of_ints = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_ints as usize);

    for _ in 0..number_of_ints {
        let value = match decode_u16(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

// 0x27 and 0x2F
pub fn decode_u32(bytes: &[u8], offset: &mut usize) -> Result<u32, String> {
    let bytes = match bytes[*offset..*offset + 4].try_into() {
        Ok(res) => res,
        Err(_) => return Err(format!("Error decoding u32 offset: {}", offset)),
    };

    let result = u32::from_le_bytes(bytes);
    *offset += std::mem::size_of::<u32>();

    Ok(result)
}

// 0x28
pub fn decode_u32_array(bytes: &[u8], offset: &mut usize) -> Result<Vec<u32>, String> {
    let number_of_ints = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_ints as usize);

    for _ in 0..number_of_ints {
        let value = match decode_u32(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

// 0x29
pub fn decode_int16(bytes: &[u8], offset: &mut usize) -> Result<i16, String> {
    let bytes = match bytes[*offset..*offset + std::mem::size_of::<i16>()].try_into() {
        Ok(res) => res,
        Err(_) => return Err(format!("Error decoding int16 offset: {}", offset)),
    };

    let result = i16::from_le_bytes(bytes);
    *offset += std::mem::size_of::<i16>();

    Ok(result)
}

// 0x2A
pub fn decode_int16_array(bytes: &[u8], offset: &mut usize) -> Result<Vec<i16>, String> {
    let number_of_ints = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_ints as usize);

    for _ in 0..number_of_ints {
        let value = match decode_int16(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

// 0x31
pub fn decode_int64(bytes: &[u8], offset: &mut usize) -> Result<i64, String> {
    let bytes = match bytes[*offset..*offset + std::mem::size_of::<i64>()].try_into() {
        Ok(res) => res,
        Err(_) => return Err(format!("Error decoding int64 offset: {}", offset)),
    };

    let result = i64::from_le_bytes(bytes);
    *offset += std::mem::size_of::<i64>();

    Ok(result)
}

// 0x32
pub fn decode_int64_array(bytes: &[u8], offset: &mut usize) -> Result<Vec<i64>, String> {
    let number_of_ints = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_ints as usize);

    for _ in 0..number_of_ints {
        let value = match decode_int64(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

// 0x33
pub fn decode_u64(bytes: &[u8], offset: &mut usize) -> Result<u64, String> {
    let bytes = match bytes[*offset..*offset + 8].try_into() {
        Ok(res) => res,
        Err(_) => return Err(format!("Error decoding u64 offset: {}", offset)),
    };

    let result = u64::from_le_bytes(bytes);
    *offset += std::mem::size_of::<u64>();

    Ok(result)
}

// 0x34
pub fn decode_u64_array(bytes: &[u8], offset: &mut usize) -> Result<Vec<u64>, String> {
    let number_of_ints = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_ints as usize);

    for _ in 0..number_of_ints {
        let value = match decode_u64(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

// 0x35
pub fn decode_bool(bytes: &[u8], offset: &mut usize) -> bool {
    let result = bytes[*offset] != 0;
    *offset += std::mem::size_of::<bool>();
    result
}

// 0x36
pub fn decode_bool_array(bytes: &[u8], offset: &mut usize) -> Result<Vec<bool>, String> {
    let number_of_bools = match decode_u32(bytes, offset) {
        Ok(res) => res as usize,
        Err(err) => return Err(err),
    };
    let mut result = vec![false; number_of_bools];

    for i in 0..number_of_bools {
        result[i] = decode_bool(bytes, offset);
    }

    Ok(result)
}

//0x37
pub fn decode_ordinal_string_list(
    bytes: &[u8],
    offset: &mut usize,
) -> Result<HashMap<u32, String>, String> {
    let length = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut values = HashMap::new();

    for _ in 0..length {
        let ordinal = match decode_u32(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        let string_value = match decode_utf8_string(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        values.insert(ordinal, string_value);
    }

    Ok(values)
}

pub fn get_ordinal_string_from_values(
    values: &std::collections::HashMap<u32, String>,
    bytes: &[u8],
    offset: &mut usize,
) -> Result<String, String> {
    let index = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    if let Some(value) = values.get(&index) {
        Ok(value.clone())
    } else {
        Ok(String::new())
    }
}

// 0x39, 0x3B, 0x3D
pub fn decode_id(bytes: &[u8], offset: &mut usize) -> Result<IDComplexType, String> {
    let mut result = IDComplexType::new();

    result.value = String::new();
    result.part_count = bytes[*offset];
    *offset += 1;

    if result.part_count == 0xFF {
        result.address = match decode_u64(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

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
            let s = match decode_u64_string(bytes, offset) {
                Ok(res) => res,
                Err(err) => return Err(err),
            };

            if i > 0 {
                result.value.push('.');
            }

            result.value.push_str(&s);
        }

        if result.part_count == 0 {
            result.value = "null".to_string();
        }
    }

    Ok(result)
}

// 0x3A, 0x3C, 0x3E
pub fn decode_id_array(bytes: &[u8], offset: &mut usize) -> Result<Vec<IDComplexType>, String> {
    let number_of_ids = match decode_u32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let mut result = Vec::with_capacity(number_of_ids as usize);

    for _ in 0..number_of_ids {
        let value = match decode_id(bytes, offset) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        result.push(value);
    }

    Ok(result)
}

// 0x41
pub fn decode_int32_vector2(bytes: &[u8], offset: &mut usize) -> Result<Int32Vector2, String> {
    let a = match decode_int32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let b = match decode_int32(bytes, offset) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    Ok(Int32Vector2 { a, b })
}
