mod audio_writer;
mod convert;
mod serde;

use convert::convert;
use rayon::prelude::*;
use std::env::args;
use std::path::Path;
use walkdir::WalkDir;

/// see https://wiibrew.org/wiki/AST_file
/// and http://wiki.tockdom.com/wiki/AST_(File_Format)
fn main() {
    let path = args().nth(1).expect("path not provided");
    let path = Path::new(&path);
    convert_all(path);
}

fn convert_all(path: &Path) {
    assert!(path.is_dir(), "convert all path is not dir");

    let paths = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.extension().unwrap_or_default() == "ast")
        .collect::<Vec<_>>();

    rayon::ThreadPoolBuilder::new().thread_name(|i| format!("rayon thread {}", i));
    paths.par_iter().for_each(|path| convert(path));
}
