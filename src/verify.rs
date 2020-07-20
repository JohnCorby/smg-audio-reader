use crate::structs::{AstHeader, AudioFormat, BlockChunkHeader};

pub trait Verifiable {
    fn verify(self) -> Self;
}

impl Verifiable for AstHeader {
    fn verify(self) -> Self {
        assert_eq!(self.magic, AstHeader::MAGIC, "wrong ast magic");
        assert_eq!(
            self.audio_format,
            AudioFormat::PCM16 as u16,
            "wrong audio format"
        );
        assert_eq!(self.bit_depth, 16, "wrong bit depth");
        self
    }
}

impl Verifiable for BlockChunkHeader {
    fn verify(self) -> Self {
        assert_eq!(
            self.magic,
            BlockChunkHeader::MAGIC,
            "wrong block chunk magic"
        );
        self
    }
}
