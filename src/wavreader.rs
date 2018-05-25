extern crate hound;

use outline::{Clip, AudioReader};
use std::io::Read;
use std::io;

impl <R, C> AudioReader<C> for hound::WavReader<R> where R: Read, C: Clip  {
    type Reader = R;
    //type ClipType = C;
    fn read(&mut self, reader: R) -> io::Result<C> {
        C::from_samples(self.samples())
        //Ok(SampleArray::new(self.spec().sample_rate as u64,
        //    self.samples().map(|x| x as i32).collect()))
    }

    //fn samples<> need to implement this
}
