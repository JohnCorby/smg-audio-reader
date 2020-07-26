use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

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

    fn read_or_pad(&mut self, buf: &mut [u8]) {
        self.read(buf).unwrap_or_default();
    }
}

impl FileExt for File {}
