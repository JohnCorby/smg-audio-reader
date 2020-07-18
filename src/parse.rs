use std::fs::File;
use std::io::{Error, Read};
use std::mem::size_of;

use bincode::{options, Options};
use serde::Deserialize;

use crate::structs::{AstFile, AstHeader, BlockChunk, BlockChunkHeader, PcmBlock};

/// something that can be parsed from a file
pub trait Parsable {
    /// construct from a file
    fn parse(file: &mut File) -> Result<Self, Error>
    where
        Self: Sized;
}

/// deserialize but with our own options
fn deserialize<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> T {
    options()
        .with_big_endian()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize(bytes)
        .expect("error deserializing bytes")
}

/// implements `from_file` for the given deserializable struct
macro_rules! impl_deserialize_parse {
    ($struct_name:ident) => {
        impl Parsable for $struct_name {
            fn parse(file: &mut File) -> Result<Self, Error> {
                let mut bytes = [0; size_of::<Self>()]; // allocated on the stack
                file.read_exact(&mut bytes)?;
                Ok(deserialize(&bytes))
            }
        }
    };
}

impl_deserialize_parse!(AstHeader);
impl_deserialize_parse!(BlockChunkHeader);

impl Parsable for AstFile {
    fn parse(file: &mut File) -> Result<Self, Error> {
        Ok(AstFile {
            header: AstHeader::parse(file)?,
            block_chunks: {
                let mut ret = Vec::new();
                while let Ok(block_chunk) = BlockChunk::parse(file) {
                    ret.push(block_chunk);
                }
                ret
            },
        })
    }
}

impl Parsable for BlockChunk {
    fn parse(file: &mut File) -> Result<Self, Error> {
        let header = BlockChunkHeader::parse(file)?;
        let num_channels = 4;
        let pcm_blocks = {
            let mut ret = Vec::with_capacity(num_channels as usize);
            for _ in 0..num_channels {
                ret.push(PcmBlock::parse(file)?)
            }
            ret
        };

        Ok(BlockChunk {
            header,
            num_channels,
            pcm_blocks,
        })
    }
}

impl Parsable for PcmBlock {
    fn parse(file: &mut File) -> Result<Self, Error> {
        let block_size: u32 = 0x2760;
        let bytes = {
            let mut bytes = vec![0; block_size as usize];
            file.read_exact(&mut bytes)?;
            bytes
        };

        Ok(PcmBlock(bytes))
    }
}
