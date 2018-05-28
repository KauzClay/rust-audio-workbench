//mod outline;
//mod outline;
use outline::{Clip, Sample};

pub struct SampleArray {
    pub samples_per_sec: u32,
    pub samples: Vec<Sample>,
}


impl SampleArray {
    pub fn new(samples_per_sec: u32, samples: Vec<Sample>) -> Self {
        SampleArray {samples_per_sec, samples}
    }
}


impl Clip for SampleArray {

    /// returns the duration in samples.
    fn duration(&self) -> u64 {
        self.samples.len() as u64
    }

    /// returns the number of samples per second of this clip.
    fn samples_per_sec(&self) -> u32 {
        self.samples_per_sec
    }

    /// get the sample at a point.
    fn get(&self, sample_at: u64) -> Sample {
        if (sample_at as usize) < self.samples.len() {
            self.samples[sample_at as usize]
        } else {
            0
        }
    }
    // should this panic or return a Result instead upon out-of-bounds access
    // consider performance

    // set the sample at a point.
    //fn set(&mut self, sample_at: u64, val: Sample) {
    //    self.samples[sample_at as usize] = val;
    //}
}
