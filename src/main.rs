use std::fs::File;
use std::io::Read;

use bincode::deserialize;
use serde::Deserialize;

fn main() {
    const PATH: &str = "data/AudioRes/Stream/SMG_astrodome02_strm.ast";

    let mut file = File::open(PATH).expect("error opening file");
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded).expect("error reading file");

    let decoded: ASTFile = deserialize(&encoded).expect("error deserializing file");
    println!("{:#?}", decoded)
}


#[derive(Deserialize, Debug)]
struct ASTFile {}
