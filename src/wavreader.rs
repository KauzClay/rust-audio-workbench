//! An implementation of our AudioReader and AudioWriter Traits using the Hound Wav crate

extern crate hound;

use std::io;
use std::io::{Read, Write, Seek};
use std::sync::Arc;
use samplearray::SampleArray;
use outline::{Sample, AudioReader, AudioWriter, Clip};


impl <R> AudioReader for hound::WavReader<R> where R: Read  {
    type Reader = R;

    fn read(&mut self) -> Option<Vec<Arc<SampleArray>>> {
        let channels = self.spec().channels as usize;
        let mut channel_samples = vec![Vec::new(); channels];
        let sample_rate = self.spec().sample_rate;

        let mut samples = self.samples::<i16>();
        let mut counter = 0;
        while let Some(s) = samples.next() {
            if let Ok(x) = s {
                channel_samples[counter as usize].push(x as Sample);
                counter = (counter + 1) % channels;
            } else {
                return None;
            }
        }

        Some(channel_samples.into_iter().map(|v| Arc::new(SampleArray::new(sample_rate, v))).collect())
    }
}

//TODO allow writing multiple channels
impl <W> AudioWriter for hound::WavWriter<W> where W: Write + Seek {
    type Writer = W;

    fn write(writer: Self::Writer, clip: Arc<Clip>) -> bool {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: clip.sample_rate(),
            bits_per_sample: 16, // TODO get num bits in Sample
            sample_format: hound::SampleFormat::Int,
        };
        if let Ok(mut writer) = hound::WavWriter::new(writer, spec) {
            for i in 0..clip.duration() {
                if writer.write_sample(clip.get(i)).is_err() {
                    return false
                }
            }
            writer.finalize().is_ok()
        } else {
            false
        }
    }
}
