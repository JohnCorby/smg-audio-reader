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
    let path = args().nth(1).expect("path not provided");
    let path = Path::new(&path);
    convert_all(path);
}

fn convert_one(path: &Path) {
    println!("converting {:?}", path);
    AstFile::open(&path).into_wav(&path.with_extension("wav"));
}

fn convert_all(path: &Path) {
    assert!(path.is_dir(), "convert all path is not dir");

    println!("converting all ast files in {:?}", path);
    for entry in path.read_dir().expect("error reading dir") {
        let path = entry.expect("error getting dir entry").path();
        if path.extension().unwrap_or_default() == "ast" {
            convert_one(&path);
        }
    }
}
