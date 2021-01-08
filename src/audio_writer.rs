use crate::convert::BlockChunk;
use crate::serde::AstHeader;
use hound::{SampleFormat, WavSpec, WavWriter};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// abstract writer for wav files
pub struct AudioWriter {
    writer: hound::WavWriter<BufWriter<File>>,
}
impl AudioWriter {
    pub fn new(path: &Path, header: &AstHeader) -> Self {
        let writer = WavWriter::create(
            path.with_extension("wav"),
            WavSpec {
                channels: header.num_channels,
                sample_rate: header.sample_rate,
                bits_per_sample: header.bit_depth,
                sample_format: SampleFormat::Int,
            },
        )
        .expect("error creating wav file");

        Self { writer }
    }

    pub fn write(&mut self, block_chunk: &BlockChunk) {
        // each block is a channel
        // and wav interleaves channels
        for sample_index in 0..block_chunk.header.num_samples() as usize {
            for block in &block_chunk.blocks {
                self.writer
                    .write_sample(block.0[sample_index])
                    .expect("error writing wav sample");
            }
        }
    }

    pub fn save(self) {
        self.writer.finalize().expect("error finalizing wav file");
    }
}
