use crate::structs::*;
use bincode::config::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Deref;

fn options() -> WithOtherTrailing<
    WithOtherIntEncoding<WithOtherEndian<DefaultOptions, BigEndian>, FixintEncoding>,
    AllowTrailing,
> {
    DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding()
        .allow_trailing_bytes()
}

type Reader = BufReader<File>;

impl AstFile {
    pub fn parse(reader: &mut Reader) -> Self {
        let header = AstHeader::parse(reader);

        let mut block_chunks = vec![];
        while let Some(block_chunk) = BlockChunk::parse(reader, header.num_channels) {
            block_chunks.push(block_chunk);
        }

        AstFile {
            header,
            block_chunks,
        }
    }
}

impl AstHeader {
    fn parse(reader: &mut Reader) -> Self {
        let header: Self = options()
            .deserialize_from(reader)
            .expect("error deserializing ast header");

        assert_eq!(&header.magic, Self::MAGIC, "wrong ast magic");
        assert_eq!(
            header.audio_format,
            AudioFormat::PCM16,
            "wrong audio format"
        );
        assert_eq!(header.bit_depth, 16, "wrong bit depth");

        header
    }
}

impl BlockChunk {
    fn parse(reader: &mut Reader, num_channels: u16) -> Option<Self> {
        let header = BlockChunkHeader::parse(reader)?;
        let blocks = (0..num_channels)
            .map(|_| Block::parse(reader, header.num_samples()))
            .collect();

        Some(BlockChunk { header, blocks })
    }
}

impl BlockChunkHeader {
    fn parse(reader: &mut Reader) -> Option<Self> {
        let header: bincode::Result<Self> = options().deserialize_from(reader);
        // return none if eof
        if let Err(ref error) = header {
            if let bincode::ErrorKind::Io(error) = error.deref() {
                if let std::io::ErrorKind::UnexpectedEof = error.kind() {
                    return None;
                }
            }
        }
        let header = header.expect("error deserializing block chunk header");

        assert_eq!(&header.magic, Self::MAGIC, "wrong block chunk magic");

        Some(header)
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
