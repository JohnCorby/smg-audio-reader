use std::fs::File;
use std::mem::size_of;
use std::path::Path;

use bincode::{options, Options};
use serde::Deserialize;

use crate::file_ext::FileExt;
use crate::structs::{
    AstFile, AstHeader, AudioFormat, Block, BlockChunk, BlockChunkHeader, Sample,
};
use crate::verify::Verifiable;

fn deserialize<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> T {
    options()
        .with_big_endian()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize(bytes)
        .expect("error deserializing bytes")
}

trait Parsable {
    fn parse(file: &mut File) -> Self;
}

impl AstFile {
    pub fn open(path: &Path) -> Self {
        // fixme make static assertions? maybe the optimizer does this for us
        assert_eq!(size_of::<AstHeader>(), 0x40, "wrong ast header size");
        assert_eq!(size_of::<AudioFormat>(), 2, "wrong audio format size");
        assert_eq!(
            size_of::<BlockChunkHeader>(),
            0x20,
            "wrong block chunk header size"
        );

        assert_eq!(
            path.extension().unwrap_or_default(),
            "ast",
            "path must be ast file"
        );
        let mut file = File::open(path).expect("error opening file");
        Self::parse(&mut file)
    }
}

impl Parsable for AstFile {
    fn parse(file: &mut File) -> Self {
        let header = AstHeader::parse(file);

        let mut block_chunks = vec![];
        // fixme use something other than eof because this is expensive i think
        while !file.at_eof() {
            block_chunks.push(BlockChunk::parse(file, header.num_channels));
        }

        AstFile {
            header,
            block_chunks,
        }
    }
}

impl Parsable for AstHeader {
    fn parse(file: &mut File) -> Self {
        let mut bytes = [0; size_of::<Self>()];
        file.read_or_pad(&mut bytes);

        deserialize::<Self>(&bytes).verify()
    }
}

impl BlockChunk {
    fn parse(file: &mut File, num_channels: u16) -> Self {
        let header = BlockChunkHeader::parse(file);
        let blocks = (0..num_channels)
            .map(|_| Block::parse(file, header.num_samples()))
            .collect();

        BlockChunk { header, blocks }
    }
}

impl Parsable for BlockChunkHeader {
    fn parse(file: &mut File) -> Self {
        let mut bytes = [0; size_of::<Self>()];
        file.read_or_pad(&mut bytes);

        deserialize::<Self>(&bytes).verify()
    }
}

impl Block {
    fn parse(file: &mut File, num_samples: u32) -> Self {
        Block((0..num_samples).map(|_| Sample::parse(file)).collect())
    }
}

impl Parsable for Sample {
    fn parse(file: &mut File) -> Self {
        let mut bytes = [0; size_of::<Self>()];
        file.read_or_pad(&mut bytes);

        Sample(i16::from_be_bytes(bytes))
    }
}
