use std::fs::File;
use std::io::Read;
use std::mem::size_of;

use bincode::{options, Options};
use byteorder::{BigEndian, ReadBytesExt};
use serde::Deserialize;

use crate::structs::{
    AstFile, AstHeader, Block, BlockChunk, BlockChunkHeader, AST_MAGIC, AUDIO_FORMAT_PCM16,
    BLOCK_CHUNK_MAGIC,
};

/// deserialize but with our own options
fn deserialize<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> T {
    options()
        .with_big_endian()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize(bytes)
        .expect("error deserializing bytes")
}

impl AstFile {
    pub fn parse(file: &mut File) -> Self {
        let header = AstHeader::parse(file);

        let mut block_chunks = vec![];
        // loop {
        block_chunks.push(BlockChunk::parse(file, &header));
        // todo break at end
        // }

        AstFile {
            header,
            block_chunks,
        }
    }
}

impl AstHeader {
    fn parse(file: &mut File) -> Self {
        let mut bytes = [0; size_of::<Self>()];
        file.read_exact(&mut bytes)
            .expect("error reading ast header");

        let ret: AstHeader = deserialize(&bytes);
        assert_eq!(ret.magic, AST_MAGIC, "wrong ast magic");
        assert_eq!(ret.audio_format, AUDIO_FORMAT_PCM16, "wrong audio format");
        assert_eq!(ret.bit_depth, 16, "wrong bit depth");
        ret
    }
}

impl BlockChunk {
    fn parse(file: &mut File, ast_header: &AstHeader) -> Self {
        let header = BlockChunkHeader::parse(file);

        let mut blocks = vec![];
        for _ in 0..ast_header.num_channels {
            blocks.push(Block::parse(file, &header))
        }

        BlockChunk { header, blocks }
    }
}

impl BlockChunkHeader {
    fn parse(file: &mut File) -> Self {
        let mut bytes = [0; size_of::<Self>()];
        file.read_exact(&mut bytes)
            .expect("error reading block chunk header");

        let ret: BlockChunkHeader = deserialize(&bytes);
        assert_eq!(ret.magic, BLOCK_CHUNK_MAGIC, "wrong block chunk magic");
        ret
    }
}

impl Block {
    fn parse(file: &mut File, header: &BlockChunkHeader) -> Self {
        let mut samples = Vec::with_capacity(header.block_size as usize);
        for _ in 0..header.block_size {
            let sample = file
                .read_u16::<BigEndian>()
                .expect("error reading block sample");
            samples.push(sample)
        }

        Block(samples)
    }
}
