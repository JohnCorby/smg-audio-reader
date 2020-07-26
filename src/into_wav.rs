use std::path::Path;

use hound::{SampleFormat, WavSpec, WavWriter};

use crate::structs::AstFile;

impl AstFile {
    pub fn into_wav(self, path: &Path) {
        // todo loop points?

        // create the writer
        let mut writer = WavWriter::create(
            path,
            WavSpec {
                channels: self.header.num_channels,
                sample_rate: self.header.sample_rate,
                bits_per_sample: self.header.bit_depth,
                sample_format: SampleFormat::Int,
            },
        )
        .expect("error creating wav file");

        // write the samples
        // blocks correspond to parts of channels
        // and wav interleaves channel data
        for block_chunk in self.block_chunks {
            for sample_index in 0..block_chunk.header.num_samples() {
                for block in &block_chunk.blocks {
                    writer
                        .write_sample(block.samples[sample_index as usize])
                        .expect("error writing wav sample");
                }
            }
        }

        // save to file
        writer.finalize().expect("error finalizing wav file")
    }
}
