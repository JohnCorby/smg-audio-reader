use std::env::args;
use std::path::Path;

use crate::structs::AstFile;

mod ext;
mod into_wav;
mod parse;
mod structs;

/// see https://wiibrew.org/wiki/AST_file
/// and http://wiki.tockdom.com/wiki/AST_(File_Format)#BLCK
fn main() {
    println!("reading ast");
    let path = args().nth(1).expect("path not provided");
    let path = Path::new(&path);
    let ast_file = AstFile::open(&path);

    println!("writing wav");
    ast_file.into_wav(&path.with_extension("wav"));
}
