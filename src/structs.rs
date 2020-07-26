use std::fs::File;
use std::io::BufReader;
use std::mem::size_of;
use std::path::Path;
use std::time::Instant;

use serde::Deserialize;

#[derive(Debug)]
pub struct AstFile {
    pub header: AstHeader,
    pub block_chunks: Vec<BlockChunk>,
}

impl AstFile {
    pub fn open(path: &Path) -> Self {
        assert_eq!(size_of::<AstHeader>(), 64, "wrong ast header size");
        assert_eq!(size_of::<AudioFormat>(), 2, "wrong audio format size");
        assert_eq!(
            size_of::<BlockChunkHeader>(),
            32,
            "wrong block chunk header size"
        );

        assert_eq!(
            path.extension().unwrap_or_default(),
            "ast",
            "path must be ast file"
        );

        let now = Instant::now();
        let file = Self::parse(&mut BufReader::new(
            File::open(path).expect("error opening file"),
        ));
        println!("parsing {:?} took {:?}", path, now.elapsed());
        file
    }
}

#[derive(Deserialize, Debug)]
pub struct AstHeader {
    pub magic: [u8; 4],
    pub total_channel_size: u32,
    pub audio_format: u16,
    pub bit_depth: u16,
    pub num_channels: u16,
    __unknown1: u16,
    pub sample_rate: u32,
    /// fixme not accurate?
    pub loop_start: u32,
    pub total_num_samples: u32,
    pub loop_end: u32,
    pub first_block_size: u32,
    __unknown2: [u8; 28],
}

impl AstHeader {
    pub const MAGIC: &'static [u8; 4] = b"STRM";
}

#[derive(Debug)]
#[repr(u16)]
#[allow(dead_code)]
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
    __padding: [u8; 24],
}

impl BlockChunkHeader {
    pub const MAGIC: &'static [u8; 4] = b"BLCK";

    pub fn num_samples(&self) -> u32 {
        self.block_size / 2
    }
}

#[derive(Debug)]
pub struct Block {
    pub samples: Vec<i16>,
}
