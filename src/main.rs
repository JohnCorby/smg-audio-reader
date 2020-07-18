use std::fs::File;

use crate::parse::Parsable;
use crate::structs::{AstFile, check_structs};
use std::io::Read;

mod structs;
mod parse;

const PATH: &str = "data/AudioRes/Stream/SMG_boss01a_strm.ast";

fn main() {
    check_structs();

    let mut file = File::open(PATH).expect("error opening file");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("error reading file");

    let decoded: AstFile = AstFile::from_bytes(&bytes);
    println!("{:#?}", decoded)
}
