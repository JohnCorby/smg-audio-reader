use std::env::args;
use std::path::Path;
use std::time::Instant;

use crate::structs::AstFile;

mod into_wav;
mod parse;
mod seek_ext;
mod structs;
mod verify;

/// see https://wiibrew.org/wiki/AST_file
/// and http://wiki.tockdom.com/wiki/AST_(File_Format)#BLCK
fn main() {
    let now = Instant::now();

    println!("reading ast");
    let path = args().nth(1).expect("path not provided");
    let path = Path::new(&path);
    let ast_file = AstFile::open(&path);

    println!("writing wav");
    ast_file.into_wav(&path.with_extension("wav"));

    println!("program took {:?}", now.elapsed());
}
