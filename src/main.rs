use std::fs::File;

use crate::parse::Parsable;
use crate::structs::AstFile;

mod structs;
mod parse;

const PATH: &str = "data/AudioRes/Stream/SMG_boss01a_strm.ast";

fn main() {
    let mut file = File::open(PATH).expect("error opening file");

    let decoded: AstFile = AstFile::from_file(&mut file);
    println!("{:#?}", decoded)
}
