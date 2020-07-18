use std::fs::File;
use std::time::Instant;

use crate::parse::Parsable;
use crate::structs::AstFile;

mod parse;
mod structs;

const PATH: &str = "data/AudioRes/Stream/SMG_boss01a_strm.ast";

fn main() {
    let mut file = File::open(PATH).expect("error opening file");

    let now = Instant::now();
    let _decoded = AstFile::parse(&mut file).expect("error getting AstFile from file");
    // println!("{:#X?}", decoded);

    println!("that took {:?}", now.elapsed())
}
