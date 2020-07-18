use bincode::{options, Options};
use serde::Deserialize;

pub trait Parsable {
    fn from_bytes<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> T {
        options()
            .with_big_endian()
            .with_fixint_encoding()
            .allow_trailing_bytes()
            .deserialize(bytes)
            .expect("error deserializing bytes")
    }
}

impl<'a, T: Deserialize<'a>> Parsable for T {}
