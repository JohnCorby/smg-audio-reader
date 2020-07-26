use std::fs::File;
use std::io::{BufReader, Read};

use crate::ext::FileExt;
use crate::structs::{AstFile, AstHeader, AudioFormat, Block, BlockChunk, BlockChunkHeader};

type Reader = BufReader<File>;

impl AstFile {
    pub fn parse(reader: &mut Reader) -> Self {
        let header = AstHeader::parse(reader);

        let mut block_chunks = vec![];
        while !reader.at_eof() {
            block_chunks.push(BlockChunk::parse(reader, header.num_channels));
        }

        AstFile {
            header,
            block_chunks,
        }
    }
}

impl AstHeader {
    fn parse(reader: &mut Reader) -> Self {
        let header: Self = reader.deserialize(&mut [0; 64]);

        assert_eq!(&header.magic, Self::MAGIC, "wrong ast magic");
        assert_eq!(
            header.audio_format,
            AudioFormat::PCM16 as u16,
            "wrong audio format"
        );
        assert_eq!(header.bit_depth, 16, "wrong bit depth");

        header
    }
}

impl BlockChunk {
    fn parse(reader: &mut Reader, num_channels: u16) -> Self {
        let header = BlockChunkHeader::parse(reader);
        let blocks = (0..num_channels)
            .map(|_| Block::parse(reader, header.num_samples()))
            .collect();

        BlockChunk { header, blocks }
    }
}

impl BlockChunkHeader {
    fn parse(reader: &mut Reader) -> Self {
        let header: Self = reader.deserialize(&mut [0; 32]);

        assert_eq!(&header.magic, Self::MAGIC, "wrong block chunk magic");

        header
    }
}

impl Block {
    fn parse(reader: &mut Reader, num_samples: u32) -> Self {
        Block {
            samples: (0..num_samples)
                .map(|_| {
                    let mut bytes = [0; 2];
                    reader.read(&mut bytes).unwrap_or_default();

                    i16::from_be_bytes(bytes)
                })
                .collect(),
        }
    }
}
