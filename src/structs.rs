use std::mem::size_of;

use serde::Deserialize;

pub fn check_structs() {
    assert_eq!(size_of::<AstHeader>(), 64, "AstHeader is wrong size");
    assert_eq!(size_of::<BlockHeader>(), 32, "BlockHeader is wrong size");
}


#[derive(Deserialize, Debug)]
pub struct AstFile(AstHeader, BlockHeader);

#[derive(Deserialize, Debug)]
pub struct AstHeader {
    /// "STRM" (0x5354524D)
    __identifier: [u8; 4],
    /// Size of all the BLCK chunks (size of the file minus 64)
    total_block_size: u32,
    /// Unknown (0x00010010)
    __unknown1: u32,
    /// Number of channels (typically 2 = stereo)
    num_channels: u16,
    /// Unknown (0xFFFF)
    __unknown2: u16,
    /// Sampling rate in Hz (typically 32000)
    sampling_rate: u32,
    /// Total number of samples
    total_num_samples: u32,
    /// Loopstart position in samples/bytes?
    loop_start_pos: u32,
    /// Unknown (typically same as entry 0x0014)
    __unknown3: u32,
    /// Block size for the first chunk? (typically 0x2760)
    first_block_size: u32,
    /// Unknown (Usually all zeros except 0x0028, which is 0x7F)
    __unknown4: [u8; 28],
}


#[derive(Deserialize, Debug)]
pub struct BlockHeader {
    /// "BLCK" (0x424C434B)
    __identifier: [u8; 4],
    /// Block size (typically 0x2760)
    block_size: u32,
    /// Padding (zero)
    __padding: [u8; 24],
}

#[derive(Deserialize, Debug)]
pub struct BlockData;
