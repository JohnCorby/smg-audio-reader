use std::fs::File;
use std::io::Read;
use std::mem::size_of;

use bincode::{options, Options};
use serde::Deserialize;

use crate::structs::{AstFile, AstHeader, BlockChunk, BlockChunkHeader};

pub trait Parsable {
    fn from_bytes<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> T {
        options()
            .with_big_endian()
            .with_fixint_encoding()
            .allow_trailing_bytes()
            .deserialize(bytes)
            .expect("error deserializing bytes")
    }

    fn from_file(file: &mut File) -> Self;
}


impl Parsable for AstFile {
    fn from_file(file: &mut File) -> Self {
        AstFile {
            header: AstHeader::from_file(file),
            block_chunks: vec![],
        }
    }
}

impl Parsable for AstHeader {
    fn from_file(file: &mut File) -> Self {
        let mut bytes = [0; size_of::<Self>()];
        file.read_exact(&mut bytes).expect("error reading file");
        AstFile::from_bytes(&bytes)
    }
}


impl Parsable for BlockChunk {
    fn from_file(file: &mut File) -> Self {
        BlockChunk {
            header: BlockChunkHeader::from_file(file),
            data_blocks: vec![],
        }
    }
}

impl Parsable for BlockChunkHeader {
    fn from_file(file: &mut File) -> Self {
        let mut bytes = [0; size_of::<Self>()];
        file.read_exact(&mut bytes).expect("error reading file");
        AstFile::from_bytes(&bytes)
    }
}
