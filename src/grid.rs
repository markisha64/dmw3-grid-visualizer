pub struct GridInfo {
    pub width: u8,
    pub height: u8,
    pub c: Vec<u8>,
}

pub struct GridOffsets {
    pub info_offset: u32,
    pub header1: u32,
    pub header2: u32,
    pub header3: u32,
    pub indices: u32,
    pub blocks_offset: u32,
}

pub struct Grid {
    pub offsets: GridOffsets,
    pub info: GridInfo,
    pub header1: Vec<u8>,
    pub header2: Vec<u8>,
    pub header3: Vec<u8>,
    pub indices: Vec<u8>,
    pub blocks: Vec<[[u8; 8]; 8]>,
}
