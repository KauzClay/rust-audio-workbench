use hound;
use outline::Clip;
use samplearray::SampleArray;
use std::io::Read;
use std::io;

impl <R> AudioReader<R> for hound::WavReader<R> {
    fn read<R: Read, C: Clip>(&self) -> io::Result<C> {
        Ok(SampleArray::new(self.spec().sample_rate,
            self.samples.map(|x| x as i32).collect())
    }
}
