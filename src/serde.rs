use crate::convert::Reader;
use bincode::config::*;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::mem::size_of;
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

#[derive(Deserialize_repr, Debug, PartialEq)]
#[repr(u16)]
pub enum AudioFormat {
    ADPCM,
    PCM16,
}

#[derive(Deserialize, Debug)]
pub struct AstHeader {
    __magic: [u8; 4],
    pub total_channel_size: u32,
    pub audio_format: AudioFormat,
    pub bit_depth: u16,
    pub num_channels: u16,
    __unknown1: u16,
    pub sample_rate: u32,
    /// not accurate
    pub total_num_samples: u32,
    pub loop_start: u32,
    pub loop_end: u32,
    pub first_block_size: u32,
    __unknown2: [u8; 28],
}
impl AstHeader {
    const MAGIC: &'static [u8; 4] = b"STRM";

    pub fn parse(reader: &mut Reader) -> Self {
        let header: Self = options()
            .deserialize_from(reader)
            .expect("error deserializing ast header");

        assert_eq!(&header.__magic, Self::MAGIC, "wrong ast magic");
        assert_eq!(
            header.audio_format,
            AudioFormat::PCM16,
            "wrong audio format"
        );
        assert_eq!(header.bit_depth, 16, "wrong bit depth");

        header
    }
}

#[derive(Deserialize, Debug)]
pub struct BlockChunkHeader {
    __magic: [u8; 4],
    pub block_size: u32,
    __padding: [u8; 24],
}
impl BlockChunkHeader {
    const MAGIC: &'static [u8; 4] = b"BLCK";

    pub const fn num_samples(&self) -> u32 {
        self.block_size / size_of::<i16>() as u32
    }

    pub fn parse(reader: &mut Reader) -> Option<Self> {
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

        assert_eq!(&header.__magic, Self::MAGIC, "wrong block chunk magic");

        Some(header)
    }
}
