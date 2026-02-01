use crate::strucs::data_sii::BsiiStructureDecodedBlock;
use crate::strucs::sii_types::DataTypeIdFormat;
use crate::utils::{decode_utils, serialize};
use std::collections::HashMap;

pub fn load_data_block_local(
    bytes: &[u8],
    stream_pos: &mut usize,
    segment: &mut BsiiStructureDecodedBlock,
    format_version: u32,
    values: &mut HashMap<u32, String>,
) -> Result<(), String> {
    segment.id = match decode_utils::decode_id(bytes, stream_pos) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    for i in 0..segment.segments.len() {
        let data_type = segment.segments[i].segment_type as i32;

        match data_type {
            x if x == DataTypeIdFormat::ArrayOfByteBool as i32 => {
                let res = match decode_utils::decode_bool_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::bool_vec_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfEncodedString as i32 => {
                let res = match decode_utils::decode_u64_string_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = res;
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfIdA as i32 => {
                let res = match decode_utils::decode_id_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::id_complex_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfIdC as i32 => {
                let res = match decode_utils::decode_id_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::id_complex_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfIdE as i32 => {
                let res = match decode_utils::decode_id_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::id_complex_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfInt32 as i32 => {
                let res = match decode_utils::decode_i32_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::i32_vec_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfSingle as i32 => {
                let res = match decode_utils::decode_single_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::f32_vec_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfUInt16 as i32 => {
                let res = match decode_utils::decode_u16_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::u16_vec_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfUInt32 as i32 => {
                let res = match decode_utils::decode_u32_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::u32_vec_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfUInt64 as i32 => {
                let res = match decode_utils::decode_u64_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::u64_vec_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfUTF8String as i32 => {
                let res = match decode_utils::decode_utf8_string_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = res;
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfVectorOf3Int32 as i32 => {
                let res = match decode_utils::decode_int32_vector3_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::i32_vec3_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfVectorOf3Single as i32 => {
                let res = match decode_utils::decode_single_vector3_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::single_vector_3_vec_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfVectorOf4Single as i32 => {
                let res = match decode_utils::decode_single_vector4_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::single_vector_4_vec_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfVectorOf8Single as i32 => {
                if format_version == 1 {
                    let res = match decode_utils::decode_single_vector7_array(bytes, stream_pos) {
                        Ok(res) => res,
                        Err(e) => return Err(e),
                    };
                    segment.segments[i].value = serialize::single_vector_7_vec_to_string_vec(&res);
                } else {
                    let res = match decode_utils::decode_single_vector8_array(bytes, stream_pos) {
                        Ok(res) => res,
                        Err(e) => return Err(e),
                    };
                    segment.segments[i].value = serialize::single_vector_8_vec_to_string_vec(&res);
                }

                continue;
            }
            x if x == DataTypeIdFormat::ByteBool as i32 => {
                let res = decode_utils::decode_bool(bytes, stream_pos);

                segment.segments[i].value = vec![res.to_string()];
                continue;
            }
            x if x == DataTypeIdFormat::EncodedString as i32 => {
                let res = match decode_utils::decode_u64_string(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![res];
                continue;
            }
            x if x == DataTypeIdFormat::IdType3 as i32 => {
                let res = match decode_utils::decode_id(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![res.value];
                continue;
            }
            x if x == DataTypeIdFormat::IdType2 as i32 => {
                let res = match decode_utils::decode_id(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![res.value];
                continue;
            }
            x if x == DataTypeIdFormat::Id as i32 => {
                let res = match decode_utils::decode_id(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![res.value];
                continue;
            }
            x if x == DataTypeIdFormat::Int32 as i32 => {
                let res = match decode_utils::decode_int32(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![res.to_string()];
                continue;
            }
            x if x == DataTypeIdFormat::Int64 as i32 => {
                let res = match decode_utils::decode_int64(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![res.to_string()];
                continue;
            }
            x if x == DataTypeIdFormat::UInt32Type2 as i32 => {
                let res = match decode_utils::decode_u32(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::u32_to_string_vec(res);
                continue;
            }
            x if x == DataTypeIdFormat::UInt32 as i32 => {
                let res = match decode_utils::decode_u32(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::u32_to_string_vec(res);
                continue;
            }
            x if x == DataTypeIdFormat::UInt64 as i32 => {
                let res = match decode_utils::decode_u64(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::u64_to_string_vec(res);
                continue;
            }
            x if x == DataTypeIdFormat::UInt16 as i32 => {
                let res = match decode_utils::decode_u16(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::u16_to_string_vec(res);
                continue;
            }
            x if x == DataTypeIdFormat::OrdinalString as i32 => {
                let res = match decode_utils::get_ordinal_string_from_values(
                    values,
                    bytes,
                    stream_pos,
                    Some(&segment.segments[i].name),
                ) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![res];
                continue;
            }
            x if x == DataTypeIdFormat::Single as i32 => {
                let res = match decode_utils::decode_single(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![serialize::single_to_string(res)];
                continue;
            }
            x if x == DataTypeIdFormat::UTF8String as i32 => {
                let res = match decode_utils::decode_utf8_string(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![res];
                continue;
            }
            x if x == DataTypeIdFormat::VectorOf2Single as i32 => {
                let res = match decode_utils::decode_single_vector2(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![serialize::single_vector_2_string(&res)];
                continue;
            }
            x if x == DataTypeIdFormat::VectorOf3Int32 as i32 => {
                let res = match decode_utils::decode_int32_vector3(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![serialize::vec3_u32_to_string(&res)];
                continue;
            }
            x if x == DataTypeIdFormat::VectorOf3Single as i32 => {
                let res = match decode_utils::decode_single_vector3(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![serialize::single_vector_3_to_string(&res)];
                continue;
            }
            x if x == DataTypeIdFormat::VectorOf4Single as i32 => {
                let res = match decode_utils::decode_single_vector4(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![serialize::single_vector_4_to_string(&res)];
                continue;
            }
            x if x == DataTypeIdFormat::VectorOf8Single as i32 => {
                if format_version == 1 {
                    let res = match decode_utils::decode_single_vector7(bytes, stream_pos) {
                        Ok(res) => res,
                        Err(e) => return Err(e),
                    };

                    segment.segments[i].value = vec![serialize::single_vector_7_to_string(&res)];
                } else {
                    let res = match decode_utils::decode_single_vector8(bytes, stream_pos) {
                        Ok(res) => res,
                        Err(e) => return Err(e),
                    };

                    segment.segments[i].value = vec![serialize::single_vector_8_to_string(&res)];
                }

                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfInt64 as i32 => {
                let res = match decode_utils::decode_int64_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::i64_vec_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfInt16 as i32 => {
                let res = match decode_utils::decode_int16_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::i16_vec_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::Int16 as i32 => {
                let res = match decode_utils::decode_int16(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::i16_to_string_vec(res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfVectorOf2Single as i32 => {
                let res = match decode_utils::decode_single_vector2_array(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = serialize::single_vector_2_vec_to_string_vec(&res);
                continue;
            }
            x if x == DataTypeIdFormat::ArrayOfVectorOf2Int32 as i32 => {
                let res = match decode_utils::decode_int32_vector2(bytes, stream_pos) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

                segment.segments[i].value = vec![serialize::vec2_u32_to_string(&res)];
                continue;
            }
            0 => {
                continue;
            }
            _ => {
                return Err(format!("Unknown data type: {}", data_type));
            }
        }
    }

    return Ok(());
}
