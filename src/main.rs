mod into_wav;
mod parse;
mod structs;

use crate::structs::AstFile;
use rayon::prelude::*;
use std::env::args;
use std::path::Path;
use std::time::Instant;
use walkdir::WalkDir;

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

    let paths = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.extension().unwrap_or_default() == "ast")
        .collect::<Vec<_>>();

    rayon::ThreadPoolBuilder::new().thread_name(|i| format!("rayon thread {}", i));
    paths.par_iter().for_each(|path| convert_one(path));

    println!("parsing all files in {:?} took {:?}", path, now.elapsed());
}
