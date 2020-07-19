use serde::Deserialize;

#[derive(Debug)]
pub struct AstFile {
    pub header: AstHeader,
    pub block_chunks: Vec<BlockChunk>,
}

#[derive(Deserialize, Debug)]
pub struct AstHeader {
    pub magic: [u8; 4],
    pub total_block_size: u32,
    pub audio_format: u16,
    pub bit_depth: u16,
    pub num_channels: u16,
    __unknown1: u16,
    pub sample_rate: u32,
    pub total_num_samples: u32,
    pub loop_start: u32,
    pub loop_end: u32,
    pub first_block_size: u32,
    __unknown2: [u8; 0x1C],
}

impl AstHeader {
    pub const MAGIC: [u8; 4] = *b"STRM";
}

#[derive(Deserialize, Debug)]
#[repr(u16)]
pub enum AudioFormat {
    ADPCM,
    PCM16,
}

#[derive(Debug)]
pub struct BlockChunk {
    pub header: BlockChunkHeader,
    pub blocks: Vec<Block>,
}

#[derive(Deserialize, Debug)]
pub struct BlockChunkHeader {
    pub magic: [u8; 4],
    pub block_size: u32,
    __padding: [u8; 0x18],
}

impl BlockChunkHeader {
    pub const MAGIC: [u8; 4] = *b"BLCK";
}

#[derive(Debug)]
pub struct Block(pub Vec<u16>);
