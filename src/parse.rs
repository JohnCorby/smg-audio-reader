use std::fs::File;
use std::io::Read;
use std::mem::size_of;

use bincode::{options, Options};
use byteorder::{BigEndian, ReadBytesExt};
use serde::Deserialize;

use crate::seek_ext::SeekExt;
use crate::structs::{AstFile, AstHeader, Block, BlockChunk, BlockChunkHeader, Sample};
use crate::verify::Verifiable;

fn deserialize<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> T {
    options()
        .with_big_endian()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize(bytes)
        .expect("error deserializing bytes")
}

pub trait Parsable {
    fn parse(file: &mut File) -> Self;
}

impl Parsable for AstFile {
    fn parse(file: &mut File) -> Self {
        let header = AstHeader::parse(file);

        let mut block_chunks = vec![];
        while !file.at_eof() {
            block_chunks.push(BlockChunk::parse(file, header.num_channels));
            // println!("{:X} {:X}", file.pos(), file.len());
        }

        file.print_pos();
        AstFile {
            header,
            block_chunks,
        }
    }
}

impl Parsable for AstHeader {
    fn parse(file: &mut File) -> Self {
        let mut bytes = [0; size_of::<Self>()];
        file.read_exact(&mut bytes)
            .expect("error reading ast header");

        deserialize::<AstHeader>(&bytes).verify()
    }
}

impl BlockChunk {
    fn parse(file: &mut File, num_channels: u16) -> Self {
        let header = BlockChunkHeader::parse(file);

        let blocks = (0..num_channels)
            .map(|_| Block::parse(file, header.block_size / 2))
            .collect();

        BlockChunk { header, blocks }
    }
}

impl Parsable for BlockChunkHeader {
    fn parse(file: &mut File) -> Self {
        let mut bytes = [0; size_of::<Self>()];
        file.read_exact(&mut bytes)
            .expect("error reading block chunk header");

        deserialize::<BlockChunkHeader>(&bytes).verify()
    }
}

impl Block {
    fn parse(file: &mut File, num_samples: u32) -> Self {
        let samples = (0..num_samples).map(|_| Sample::parse(file)).collect();

        Block(samples)
    }
}

impl Parsable for Sample {
    fn parse(file: &mut File) -> Self {
        let sample = file.read_u16::<BigEndian>().expect("error reading sample");

        Sample(sample)
    }
}
