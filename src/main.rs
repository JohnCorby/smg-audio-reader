use crate::structs::AstFile;
use rayon::prelude::*;
use std::env::args;
use std::path::Path;
use std::time::Instant;

mod ext;
mod into_wav;
mod parse;
mod structs;

/// see https://wiibrew.org/wiki/AST_file
/// and http://wiki.tockdom.com/wiki/AST_(File_Format)
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
    let now = Instant::now();

    let paths = path
        .read_dir()
        .expect("error reading dir")
        .map(|entry| entry.expect("error getting dir entry").path())
        .filter(|path| path.extension().unwrap_or_default() == "ast")
        .collect::<Vec<_>>();

    paths.par_iter().for_each(|path| convert_one(path));

    println!("parsing all files in {:?} took {:?}", path, now.elapsed());
}
