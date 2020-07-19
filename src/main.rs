use std::env::args;
use std::fs::File;
use std::mem::size_of;
use std::path::Path;
use std::time::Instant;

use crate::structs::{AstFile, AstHeader, AudioFormat, BlockChunkHeader};

mod parse;
mod structs;

/// see https://wiibrew.org/wiki/AST_file
/// and http://wiki.tockdom.com/wiki/AST_(File_Format)#BLCK
fn main() {
    let now = Instant::now();

    assert_eq!(size_of::<AstHeader>(), 0x40, "wrong ast header size");
    assert_eq!(size_of::<AudioFormat>(), 2, "wrong audio format size");
    assert_eq!(
        size_of::<BlockChunkHeader>(),
        0x20,
        "wrong block chunk header size"
    );

    let path = args().nth(1).expect("path not provided");
    let path = Path::new(&path);
    assert_eq!(
        path.extension().unwrap_or_default(),
        "ast",
        "path must be ast file"
    );
    let mut file = File::open(path).expect("error opening file");

    let decoded = AstFile::parse(&mut file);
    // println!("{:X?}", decoded);

    println!("that took {:?}", now.elapsed());
}
