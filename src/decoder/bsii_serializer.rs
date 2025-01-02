use crate::strucs::data_sii::{BSIIData, BsiiDataSegment, BsiiStructureDecodedBlock};
use crate::strucs::sii_types::DataTypeIdFormat;

static IDENT: &str = " ";
const LIMITED_ALPHABET: [char; 63] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z', '_',
];

fn segment_serialize(block: &BsiiStructureDecodedBlock, header_version: u32) -> String {
    let mut str_build = String::new();

    for segment in &block.segments {
        if segment.segment_type != 0 {
            let data_type = segment.segment_type as i32;

            match data_type {
                x if x == DataTypeIdFormat::ArrayOfByteBool as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfEncodedString as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfIdA as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfIdC as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfIdE as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfInt32 as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfSingle as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfUInt16 as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfUInt32 as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfUInt64 as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfUTF8String as i32 => {
                    str_build.push_str(&serialize_utf8_string_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfVectorOf3Int32 as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfVectorOf3Single as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfVectorOf4Single as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfVectorOf8Single as i32 => {
                    if header_version == 1 {
                        str_build.push_str(&serialize_vec(&segment));
                    } else {
                        str_build.push_str(&serialize_vec(&segment));
                    }
                    continue;
                }
                x if x == DataTypeIdFormat::ByteBool as i32 => {
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::EncodedString as i32 => {
                    str_build.push_str(&serialize_single_value_string(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::IdType3 as i32 => {
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::IdType2 as i32 => {
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::Id as i32 => {
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::Int32 as i32 => {
                    // There are doubts with “nil” values if value is "null"
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::Int64 as i32 => {
                    // There are doubts with “nil” values if value is "null"
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::UInt32Type2 as i32 => {
                    // There are doubts with “nil” values if value is "null"
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::UInt32 as i32 => {
                    // There are doubts with “nil” values if value is "null"
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::UInt64 as i32 => {
                    // There are doubts with “nil” values if value is "null"
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::UInt16 as i32 => {
                    // There are doubts with “nil” values if value is "null"
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::OrdinalString as i32 => {
                    str_build.push_str(&serialize_single_value_string(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::Single as i32 => {
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::UTF8String as i32 => {
                    str_build.push_str(&serialize_utf8_string(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::VectorOf2Single as i32 => {
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::VectorOf3Int32 as i32 => {
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::VectorOf3Single as i32 => {
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::VectorOf4Single as i32 => {
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::VectorOf8Single as i32 => {
                    if header_version == 1 {
                        str_build.push_str(&serialize_single_value(&segment));
                    } else {
                        str_build.push_str(&serialize_single_value(&segment));
                    }
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfInt64 as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfVectorOf2Single as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::Int16 as i32 => {
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfInt16 as i32 => {
                    str_build.push_str(&serialize_vec(&segment));
                    continue;
                }
                x if x == DataTypeIdFormat::ArrayOfVectorOf2Int32 as i32 => {
                    str_build.push_str(&serialize_single_value(&segment));
                    continue;
                }
                0 => {
                    continue;
                }
                _ => {
                    panic!("Unknown data type: {}", data_type);
                }
            }
        }
    }

    str_build
}

fn serialize_vec(data: &BsiiDataSegment) -> String {
    let value = &data.value;
    let mut text = String::new();

    text.push_str(&format!("{}{}: {}\n", IDENT, data.name, value.len()));

    for (i, val) in value.iter().enumerate() {
        text.push_str(&format!("{}{}[{}]: {}\n", IDENT, data.name, i, val));
    }

    text
}

fn serialize_single_value(data: &BsiiDataSegment) -> String {
    let value = &data.value;

    format!("{}{}: {}\n", IDENT, data.name, value[0])
}

fn serialize_single_value_string(data: &BsiiDataSegment) -> String {
    let value = &data.value[0];

    let value_compare = match value {
        /*
        Support for external decryptors (more data types possible)
        */
        v if data.name == "part_type" && v == "vehicle" => "unknown".to_string(),
        v if data.name == "type" && v == "parking" => "spot".to_string(),
        v if data.name == "setup" && v == "low_beam" => "candela_hue_saturation".to_string(),
        v if data.name == "setup" && v == "parking" => "lumen_hue_saturation".to_string(),
        v if data.name == "dir_type" && v == "parking" => "wide".to_string(),
        v if data.name == "dir_type" && v == "low_beam" => "narrow".to_string(),
        v if data.name == "cut_direction" && v.is_empty() => "forward".to_string(),
        /*
        Standard types
        */
        v if v.is_empty() => "\"\"".to_string(),
        _ => value.to_string(),
    };

    format!("{}{}: {}\n", IDENT, data.name, value_compare)
}

fn serialize_utf8_string_vec(data: &BsiiDataSegment) -> String {
    let value = &data.value;
    let mut text = String::new();

    text.push_str(&format!("{}{}: {}\n", IDENT, data.name, value.len()));

    for (i, val) in value.iter().enumerate() {
        if val.parse::<u32>().is_ok() {
            text.push_str(&format!("{}{}[{}]: {}\n", IDENT, data.name, i, val));
        } else {
            if val.is_empty() {
                text.push_str(&format!("{}{}[{}]: \"\"\n", IDENT, data.name, i));
            } else {
                if is_limited_alphabet(val) {
                    text.push_str(&format!("{}{}[{}]: {}\n", IDENT, data.name, i, val));
                } else {
                    text.push_str(&format!("{}{}[{}]: \"{}\"\n", IDENT, data.name, i, val));
                }
            }
        }
    }

    text
}

fn serialize_utf8_string(data: &BsiiDataSegment) -> String {
    let value = &data.value[0];
    let mut text = format!("{}{}: ", IDENT, data.name);

    if let Ok(_) = value.parse::<i32>() {
        text.push_str(&format!("{}\n", value));
    } else {
        if value.is_empty() {
            text.push_str("\"\"\n");
        } else {
            if is_limited_alphabet(value) {
                text.push_str(&format!("{}\n", value));
            } else {
                text.push_str(&format!("\"{}\"\n", value));
            }
        }
    }

    text
}

fn is_limited_alphabet(value: &str) -> bool {
    for c in value.chars() {
        if !LIMITED_ALPHABET.contains(&c) {
            return false;
        }
    }

    true
}

pub fn serializer(data: BSIIData) -> Vec<u8> {
    let mut str_build: String = String::new();

    str_build.push_str("SiiNunit\n");
    str_build.push_str("{\n");

    for block in data.decoded_blocks {
        if block.name.is_empty() || block.id.value.is_empty() {
            continue;
        }

        let res_string = segment_serialize(&block, data.header.version);

        str_build.push_str(format!("{} : {} {{\n", block.name, block.id.value).as_str());
        str_build.push_str(&res_string);
        str_build.push_str("}\n\n");
    }
    str_build.push_str("}");

    str_build.into_bytes()
}
