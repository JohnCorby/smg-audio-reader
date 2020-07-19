use serde::Deserialize;

#[derive(Debug)]
pub struct AstFile {
    pub header: AstHeader,
    pub block_chunks: Vec<BlockChunk>,
}

pub const AST_MAGIC: u32 = 0x5354524D;

#[derive(Deserialize, Debug)]
pub struct AstHeader {
    pub magic: u32,
    pub total_block_size: u32,
    pub audio_format: u16,
    pub bit_depth: u16,
    pub num_channels: u16,
    __unknown1: u16,
    pub sample_rate: u32,
    pub num_samples: u32,
    pub loop_start: u32,
    pub loop_end: u32,
    pub first_block_size: u32,
    __unknown2: [u8; 0x1C],
}

// pub const AUDIO_FORMAT_ADPCM: u16 = 0;
pub const AUDIO_FORMAT_PCM16: u16 = 1;

pub const BLOCK_CHUNK_MAGIC: u32 = 0x424C434B;

#[derive(Debug)]
pub struct BlockChunk {
    pub header: BlockChunkHeader,
    pub blocks: Vec<Block>,
}

#[derive(Deserialize, Debug)]
pub struct BlockChunkHeader {
    pub magic: u32,
    pub block_size: u32,
    __padding: [u8; 0x18],
}

#[derive(Debug)]
pub struct Block(pub Vec<u16>);
