use crate::decoder::bsii_serializer;
use crate::decoder::load_data_block::load_data_block_local;
use crate::strucs::data_sii::{
    BSIIData, BsiiDataSegment, BsiiStructureBlock, BsiiSupportedVersions, IDComplexType,
};
use crate::strucs::sii_types::DataTypeIdFormat;
use crate::utils::decode_utils;
use std::collections::HashMap;

fn check_version(file_data: &BSIIData) -> bool {
    file_data.header.version != BsiiSupportedVersions::Version1 as u32
        && file_data.header.version != BsiiSupportedVersions::Version2 as u32
        && file_data.header.version != BsiiSupportedVersions::Version3 as u32
}

fn read_data_block(bytes: &[u8], stream_pos: &mut usize) -> BsiiDataSegment {
    let mut result = BsiiDataSegment {
        name: String::new(),
        segment_type: 0,
        value: Vec::new(),
        ordinal_string_hash: None,
    };

    result.segment_type = decode_utils::decode_u32(bytes, stream_pos);
    if result.segment_type != 0 {
        result.name = decode_utils::decode_utf8_string(bytes, stream_pos);
    }

    // IF THE TYPE IS 55
    if result.segment_type == DataTypeIdFormat::OrdinalString as u32 {
        // READ THE ORDINAL STRING LIST NOW
        result.ordinal_string_hash =
            Some(decode_utils::decode_ordinal_string_list(bytes, stream_pos));
    }

    result
}

pub fn decode(file_bin: &[u8]) -> Option<Vec<u8>> {
    let mut stream_pos = 0;

    let mut file_data = BSIIData::new();
    file_data.header.signature = decode_utils::decode_u32(&file_bin, &mut stream_pos);
    file_data.header.version = decode_utils::decode_u32(&file_bin, &mut stream_pos);

    let mut current_block: BsiiStructureBlock;
    let mut block_type: u32;
    let mut ordinal_lists: HashMap<u32, HashMap<u32, String>> = HashMap::new();

    let mut stream_pos: usize = 0;
    let mut file_data = BSIIData::new();

    file_data.header.signature = decode_utils::decode_u32(&file_bin, &mut stream_pos);
    file_data.header.version = decode_utils::decode_u32(&file_bin, &mut stream_pos);

    if check_version(&file_data) {
        return None;
    }

    loop {
        if stream_pos >= file_bin.len() {
            break;
        }

        block_type = decode_utils::decode_u32(&file_bin, &mut stream_pos);

        if block_type == 0 {
            current_block = BsiiStructureBlock::new();
            current_block.block_type = block_type;
            current_block.validity = decode_utils::decode_bool(&file_bin, &mut stream_pos);

            if !current_block.validity {
                file_data.blocks.push(current_block);
                continue;
            }

            current_block.structure_id = decode_utils::decode_u32(&file_bin, &mut stream_pos);
            current_block.name = decode_utils::decode_utf8_string(&file_bin, &mut stream_pos);

            let mut segment_type = 999;

            while segment_type != 0 {
                let segment_data = read_data_block(&file_bin, &mut stream_pos);
                segment_type = segment_data.segment_type;

                if segment_data.segment_type == DataTypeIdFormat::OrdinalString as u32
                    && !ordinal_lists.contains_key(&current_block.structure_id)
                {
                    let string_hash = match segment_data.ordinal_string_hash.clone() {
                        Some(res) => res,
                        None => return None,
                    };

                    ordinal_lists.insert(current_block.structure_id, string_hash);
                }

                current_block.segments.push(segment_data);
            }

            if !file_data
                .blocks
                .iter()
                .any(|x| x.structure_id == current_block.structure_id)
            {
                file_data.blocks.push(current_block);
            }
        } else {
            let block_data_item = match file_data
                .blocks
                .iter()
                .find(|block| block.structure_id == block_type)
            {
                Some(block) => block,
                None => return None,
            };

            let mut block_data = BsiiStructureBlock::new();

            block_data.structure_id = block_data_item.structure_id;
            block_data.name = block_data_item.name.clone();
            block_data.block_type = block_data_item.block_type;
            block_data.validity = block_data_item.validity;

            for segment in &block_data_item.segments {
                block_data.segments.push(BsiiDataSegment {
                    name: segment.name.clone(),
                    segment_type: segment.segment_type,
                    value: segment.value.clone(),
                    ordinal_string_hash: segment.ordinal_string_hash.clone(),
                });
            }

            if !block_data_item.id.value.is_empty() {
                block_data.id = IDComplexType {
                    part_count: block_data_item.id.part_count,
                    address: block_data_item.id.address,
                    value: block_data_item.id.value.clone(),
                };
            }

            let mut list: HashMap<u32, String> = HashMap::new();

            if let Some(existing_list) = ordinal_lists.get(&block_data.structure_id) {
                list = existing_list.clone();
            }

            load_data_block_local(
                &file_bin,
                &mut stream_pos,
                &mut block_data,
                file_data.header.version,
                &mut list,
            );

            file_data.decoded_blocks.push(block_data);
        }
    }

    match bsii_serializer::serializer(file_data) {
        Some(res) => Some(res),
        None => return None,
    }
}
