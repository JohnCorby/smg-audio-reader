use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;

use bincode::{options, Options};
use byteorder::{BigEndian, ReadBytesExt};
use serde::Deserialize;

use crate::structs::{AstFile, AstHeader, AudioFormat, Block, BlockChunk, BlockChunkHeader};

/// deserialize but with our own options
fn deserialize<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> T {
    options()
        .with_big_endian()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize(bytes)
        .expect("error deserializing bytes")
}

macro_rules! print_pos {
    ($file:expr) => {
        println!(
            "file pos: {:X}",
            $file
                .seek(SeekFrom::Current(0))
                .expect("error getting file pos")
        );
    };
}

impl AstFile {
    pub fn parse(file: &mut File) -> Self {
        let header = AstHeader::parse(file);

        let mut block_chunks = vec![];
        loop {
            block_chunks.push(BlockChunk::parse(file, &header));
            // todo break at end
        }

        AstFile {
            header,
            block_chunks,
        }
    }
}

impl AstHeader {
    fn parse(file: &mut File) -> Self {
        print_pos!(file);
        let mut bytes = [0; size_of::<Self>()];
        file.read_exact(&mut bytes)
            .expect("error reading ast header");

        let ret: AstHeader = deserialize(&bytes);
        // todo move verifying code to own file
        assert_eq!(ret.magic, AstHeader::MAGIC, "wrong ast magic");
        assert_eq!(
            ret.audio_format,
            AudioFormat::PCM16 as u16,
            "wrong audio format"
        );
        assert_eq!(ret.bit_depth, 16, "wrong bit depth");
        println!("got ast header {:?}", ret);
        print_pos!(file);
        ret
    }
}

impl BlockChunk {
    fn parse(file: &mut File, parent_header: &AstHeader) -> Self {
        let header = BlockChunkHeader::parse(file);

        let blocks = (0..parent_header.num_channels)
            .map(|_| Block::parse(file, &header))
            .collect();

        BlockChunk { header, blocks }
    }
}

impl BlockChunkHeader {
    fn parse(file: &mut File) -> Self {
        print_pos!(file);
        let mut bytes = [0; size_of::<Self>()];
        file.read_exact(&mut bytes)
            .expect("error reading block chunk header");

        let ret: BlockChunkHeader = deserialize(&bytes);
        assert_eq!(
            ret.magic,
            BlockChunkHeader::MAGIC,
            "wrong block chunk magic"
        );
        println!("got block chunk header {:?}", ret);
        print_pos!(file);
        ret
    }
}

impl Block {
    fn parse(file: &mut File, parent_header: &BlockChunkHeader) -> Self {
        print_pos!(file);
        let num_samples = parent_header.block_size as usize / 2;
        let samples = (0..num_samples)
            .map(|_| {
                file.read_u16::<BigEndian>()
                    .expect("error reading block sample")
            })
            .collect();

        let ret = Block(samples);
        println!("got block of {} samples", ret.0.len());
        print_pos!(file);
        ret
    }
}
