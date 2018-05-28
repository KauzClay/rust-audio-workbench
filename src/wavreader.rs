extern crate hound;

use std::io;
use std::io::{Read, Write, Seek};
use std::rc::Rc;
use samplearray::SampleArray;
use outline::{Sample, AudioReader, AudioWriter, Clip};


impl <R> AudioReader for hound::WavReader<R> where R: Read  {
    type Reader = R;
    //type ClipType = C;
    fn read(&mut self) -> Option<Rc<SampleArray>> {
        let mut is_ok = true;
        let sa = Some(Rc::new(SampleArray::new(self.spec().sample_rate,
            self.samples().map(|x_result: hound::Result<Sample> | {
                if let Ok(x) = x_result { x as Sample }
                else { is_ok = false; 0 }
            }).collect())));
        if is_ok { sa }
        else { None }
    }
}


impl <W> AudioWriter for hound::WavWriter<W> where W: Write + Seek {
    type Writer = W;
    
    fn write(writer: Self::Writer, clip: Rc<Clip>) -> bool {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: clip.samples_per_sec(),
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


