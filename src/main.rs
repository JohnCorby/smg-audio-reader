use std::fs::File;
use std::time::Instant;

use crate::structs::{AstFile, AstHeader, BlockChunkHeader};
use std::env::args;
use std::mem::size_of;

mod parse;
mod structs;

/// see https://wiibrew.org/wiki/AST_file
/// and http://wiki.tockdom.com/wiki/AST_(File_Format)#BLCK
fn main() {
    assert_eq!(size_of::<AstHeader>(), 0x40, "wrong ast header size");
    assert_eq!(
        size_of::<BlockChunkHeader>(),
        0x20,
        "wrong block chunk header size"
    );

    let path = args().next().expect("you must specify a path");
    let mut file = File::open(path).expect("error opening file");

    let now = Instant::now();
    let decoded = AstFile::parse(&mut file);
    println!("{:X?}", decoded);

    println!("that took {:?}", now.elapsed());
}
