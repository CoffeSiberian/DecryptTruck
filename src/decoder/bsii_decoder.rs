use crate::decoder::bsii_serializer;
use crate::decoder::load_data_block::load_data_block_local;
use crate::strucs::data_sii::{
    BSIIData, BsiiDataSegment, BsiiStructureBlock, BsiiStructureDecodedBlock,
    BsiiSupportedVersions, IDComplexType,
};
use crate::strucs::sii_types::DataTypeIdFormat;
use crate::utils::decode_utils;
use std::collections::HashMap;

fn check_version(file_data: &BSIIData) -> bool {
    file_data.header.version != BsiiSupportedVersions::Version1 as u32
        && file_data.header.version != BsiiSupportedVersions::Version2 as u32
        && file_data.header.version != BsiiSupportedVersions::Version3 as u32
}

fn read_data_block(bytes: &[u8], stream_pos: &mut usize) -> Result<BsiiDataSegment, String> {
    let mut result = BsiiDataSegment {
        name: String::new(),
        segment_type: 0,
        value: Vec::new(),
        ordinal_string_hash: None,
    };

    result.segment_type = match decode_utils::decode_u32(bytes, stream_pos) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    if result.segment_type != 0 {
        result.name = match decode_utils::decode_utf8_string(bytes, stream_pos) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };
    }

    // IF THE TYPE IS 55
    if result.segment_type == DataTypeIdFormat::OrdinalString as u32 {
        // READ THE ORDINAL STRING LIST NOW
        let ordinal_string = match decode_utils::decode_ordinal_string_list(bytes, stream_pos) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };
        result.ordinal_string_hash = Some(ordinal_string);
    }

    Ok(result)
}

fn structure_block(
    file_bin: &[u8],
    stream_pos: &mut usize,
    ordinal_lists: &mut HashMap<u32, HashMap<u32, String>>,
    file_data: &mut BSIIData,
    block_type: u32,
) -> Result<(), String> {
    let mut current_block = BsiiStructureBlock::new();
    current_block.block_type = block_type;
    current_block.validity = decode_utils::decode_bool(&file_bin, stream_pos);

    if !current_block.validity {
        file_data.blocks.push(current_block);
        return Ok(());
    }

    current_block.structure_id = match decode_utils::decode_u32(&file_bin, stream_pos) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    current_block.name = match decode_utils::decode_utf8_string(&file_bin, stream_pos) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let mut segment_type = 999;

    while segment_type != 0 {
        let segment_data = match read_data_block(&file_bin, stream_pos) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };
        segment_type = segment_data.segment_type;

        if segment_data.segment_type == DataTypeIdFormat::OrdinalString as u32
            && !ordinal_lists.contains_key(&current_block.structure_id)
        {
            let string_hash = match segment_data.ordinal_string_hash.clone() {
                Some(res) => res,
                None => return Err("Ordinal string hash is empty".to_string()),
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

    Ok(())
}

fn data_block(
    file_bin: &[u8],
    stream_pos: &mut usize,
    ordinal_lists: &mut HashMap<u32, HashMap<u32, String>>,
    file_data: &mut BSIIData,
    block_type: u32,
    decode_block_count: &mut u32,
) -> Result<(), String> {
    *decode_block_count += 1;

    let mut block_data_item = match file_data
        .blocks
        .iter()
        .find(|block| block.structure_id == block_type)
    {
        Some(block) => BsiiStructureDecodedBlock {
            order_pos: *decode_block_count,
            id: block.id.clone(),
            structure_id: block.structure_id,
            name: block.name.clone(),
            segments: block.segments.clone(),
        },
        None => return Err("Block not found".to_string()),
    };

    /*

    for segment in &block_data_item.segments {
        block_data.segments.push(BsiiDataSegment {
            name: segment.name.clone(),
            segment_type: segment.segment_type,
            value: segment.value.clone(),
            ordinal_string_hash: segment.ordinal_string_hash.clone(),
        });
    }

    */

    /*

    I'm not entirely sure if this is really necessary but
    according to the tests these pass without a problem.

    */

    if !block_data_item.id.value.is_empty() {
        block_data_item.id = IDComplexType {
            part_count: block_data_item.id.part_count,
            address: block_data_item.id.address,
            value: block_data_item.id.value.clone(),
        };
    }

    let mut list: HashMap<u32, String> = HashMap::new();

    if let Some(existing_list) = ordinal_lists.get(&block_data_item.structure_id) {
        list = existing_list.clone();
    }

    match load_data_block_local(
        &file_bin,
        stream_pos,
        &mut block_data_item,
        file_data.header.version,
        &mut list,
    ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    file_data.decoded_blocks.push(block_data_item);
    Ok(())
}

pub fn decode(file_bin: &[u8]) -> Result<Vec<u8>, String> {
    let mut block_type: u32;
    let mut ordinal_lists: HashMap<u32, HashMap<u32, String>> = HashMap::new();

    let mut stream_pos: usize = 0;
    let mut file_data = BSIIData::new();

    file_data.header.signature = match decode_utils::decode_u32(&file_bin, &mut stream_pos) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    file_data.header.version = match decode_utils::decode_u32(&file_bin, &mut stream_pos) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    if check_version(&file_data) {
        return Err("Unsupported version".to_string());
    }

    loop {
        if stream_pos >= file_bin.len() {
            return Err("End of file".to_string());
        }

        block_type = match decode_utils::decode_u32(&file_bin, &mut stream_pos) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        if block_type == 0 {
            match structure_block(
                &file_bin,
                &mut stream_pos,
                &mut ordinal_lists,
                &mut file_data,
                block_type,
            ) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else {
            break;
        }
    }

    let mut decode_block_count: u32 = 0;
    let mut firt_load_block = true;
    loop {
        if stream_pos >= file_bin.len() {
            break;
        }
        if firt_load_block {
            firt_load_block = false;
        } else {
            block_type = match decode_utils::decode_u32(&file_bin, &mut stream_pos) {
                Ok(res) => res,
                Err(e) => return Err(e),
            };
        }

        if block_type == 0 {
            let mut current_block = BsiiStructureBlock::new();
            current_block.block_type = block_type;
            current_block.validity = decode_utils::decode_bool(&file_bin, &mut stream_pos);

            if !current_block.validity {
                file_data.blocks.push(current_block);
                break;
            }
        }

        match data_block(
            &file_bin,
            &mut stream_pos,
            &mut ordinal_lists,
            &mut file_data,
            block_type,
            &mut decode_block_count,
        ) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    Ok(bsii_serializer::serializer(&file_data))
}
