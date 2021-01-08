use crate::audio_writer::AudioWriter;
use crate::serde::{AstHeader, AudioFormat, BlockChunkHeader};
use std::fs::File;
use std::io::{BufReader, Read};
use std::mem::size_of;
use std::path::Path;
use std::time::Instant;

pub type Reader = BufReader<File>;

/// take an ast file and convert it into a wav file
pub fn convert(path: &Path) {
    let now = Instant::now();

    // static size checks lol
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

    let file = File::open(path).expect("error opening file");
    let mut reader = BufReader::new(file);

    let header = AstHeader::parse(&mut reader);
    let mut writer = AudioWriter::new(path, &header);
    while let Some(block_chunk) = BlockChunk::parse(&mut reader, &header) {
        writer.write(&block_chunk)
    }
    writer.save();

    println!("converting {:?} took {:?}", path, now.elapsed());
}

#[derive(Debug)]
pub struct BlockChunk {
    pub header: BlockChunkHeader,
    pub blocks: Vec<Block>,
}
impl BlockChunk {
    fn parse(reader: &mut Reader, ast_header: &AstHeader) -> Option<Self> {
        let block_chunk_header = BlockChunkHeader::parse(reader)?;
        let blocks = (0..ast_header.num_channels)
            .map(|_| Block::parse(reader, &block_chunk_header))
            .collect();

        Some(BlockChunk {
            header: block_chunk_header,
            blocks,
        })
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Block(pub Vec<i16>);
impl Block {
    fn parse(reader: &mut Reader, header: &BlockChunkHeader) -> Self {
        Block(
            (0..header.num_samples())
                .map(|_| {
                    let mut bytes = [0; 2];
                    reader.read(&mut bytes).unwrap_or_default();

                    i16::from_be_bytes(bytes)
                })
                .collect(),
        )
    }
}
