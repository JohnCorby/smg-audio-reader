use std::io::{Read, Seek, SeekFrom};

use bincode::{options, Options};
use serde::Deserialize;

pub trait FileExt: Seek + Read {
    fn print_pos(&mut self) {
        println!("file pos: {:X}", self.pos());
    }

    fn pos(&mut self) -> u64 {
        self.seek(SeekFrom::Current(0))
            .expect("error getting current pos")
    }

    fn at_eof(&mut self) -> bool {
        let current = self.pos();
        let end = self.seek(SeekFrom::End(0)).expect("error seeking to end");

        if current == end {
            true
        } else {
            self.seek(SeekFrom::Start(current))
                .expect("error seeking back to current");
            false
        }
    }

    fn deserialize<'a, T: Deserialize<'a>>(&mut self, bytes: &'a mut [u8]) -> T {
        self.read(bytes).unwrap_or_default();

        options()
            .with_big_endian()
            .with_fixint_encoding()
            .allow_trailing_bytes()
            .deserialize(bytes)
            .expect("error deserializing bytes")
    }
}

impl<T: Seek + Read> FileExt for T {}
