use std::collections::HashMap;

pub enum SignatureType {
    PlainText = 1315531091,
    Encrypted = 1131635539,
    Binary = 1229542210,
    _3nK = 21720627,
}

pub struct SIIHeader {
    pub signature: u32,
    pub data_size: u32,
}

#[allow(dead_code)]
pub struct SIIData {
    pub header: SIIHeader,
    pub data: Vec<u8>,
}

pub enum BsiiSupportedVersions {
    Version1 = 1,
    Version2 = 2,
    Version3 = 3,
}

pub struct BSIIData {
    pub header: BSIIHeader,
    pub blocks: Vec<BsiiStructureBlock>,
    pub decoded_blocks: Vec<BsiiStructureBlock>,
}

pub struct IDComplexType {
    pub part_count: u8,
    pub address: u64,
    pub value: String,
}

pub struct BSIIHeader {
    pub signature: u32,
    pub version: u32,
}

pub struct BsiiStructureBlock {
    pub block_type: u32,
    pub structure_id: u32,
    pub validity: bool,
    pub name: String,
    pub segments: Vec<BsiiDataSegment>,
    pub id: IDComplexType,
}

pub struct BsiiDataSegment {
    pub name: String,
    pub segment_type: u32,
    pub value: Vec<String>,
    pub ordinal_string_hash: Option<HashMap<u32, String>>,
}

impl BSIIData {
    pub fn new() -> Self {
        BSIIData {
            header: BSIIHeader::new(),
            blocks: Vec::new(),
            decoded_blocks: Vec::new(),
        }
    }
}

impl BSIIHeader {
    pub fn new() -> Self {
        BSIIHeader {
            signature: 0,
            version: 0,
        }
    }
}

impl BsiiStructureBlock {
    pub fn new() -> Self {
        BsiiStructureBlock {
            block_type: 0,
            structure_id: 0,
            validity: false,
            name: String::new(),
            segments: Vec::new(),
            id: IDComplexType::new(),
        }
    }
}

impl IDComplexType {
    pub fn new() -> Self {
        IDComplexType {
            part_count: 0,
            address: 0,
            value: String::new(),
        }
    }
}

impl SIIHeader {
    pub fn new() -> Self {
        SIIHeader {
            signature: 999,
            data_size: 0,
        }
    }
}
