use serde::Deserialize;

#[derive(Debug)]
pub struct AstFile {
    pub header: AstHeader,
    pub block_chunks: Vec<BlockChunk>,
}

#[derive(Deserialize, Debug)]
pub struct AstHeader {
    /// "STRM" (0x5354524D)
    __identifier: u32,
    /// Size of all the BLCK chunks (size of the file minus 64)
    pub total_block_size: u32,
    /// Unknown (0x00010010)
    __unknown1: u32,
    /// Number of channels (typically 2 = stereo)
    pub num_channels: u16,
    /// Unknown (0xFFFF)
    __unknown2: u16,
    /// Sampling rate in Hz (typically 32000)
    pub sampling_rate: u32,
    /// Total number of samples
    pub total_num_samples: u32,
    /// Loopstart position in samples/bytes?
    pub loop_start_pos: u32,
    /// Unknown (typically same as entry 0x0014)
    __unknown3: u32,
    /// Block size for the first chunk? (typically 0x2760)
    pub first_block_size: u32,
    /// Unknown (Usually all zeros except 0x0028, which is 0x7F)
    __unknown4: [u8; 28],
}

#[derive(Debug)]
pub struct BlockChunk {
    pub header: BlockChunkHeader,
    pub num_channels: u16,
    /// PCM16 data blocks
    pub pcm_blocks: Vec<PcmBlock>,
}

#[derive(Deserialize, Debug)]
pub struct BlockChunkHeader {
    /// "BLCK" (0x424C434B)
    __identifier: u32,
    /// Block size (typically 0x2760)
    pub block_size: u32,
    /// Padding (zero)
    __padding: [u8; 24],
}

#[derive(Debug)]
pub struct PcmBlock(pub Vec<u8>);

// mod fmt {
//     use std::fmt::{Debug, Error, Formatter, Result};
//     use std::str::{from_utf8, from_utf8_unchecked};
//
//     use crate::structs::PcmBlock;
//
//     fn bytes_to_str(bytes: &[u8]) -> std::result::Result<&str, Error> {
//         from_utf8(bytes).map_err(|_| Error)
//     }
//
//     impl Debug for PcmBlock {
//         fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//             // let str = bytes_to_str(&self.0)?;
//             let str = from_utf8_unchecked(&self.0).expect("error converting chars to str");
//             f.write_str(str)
//         }
//     }
// }
