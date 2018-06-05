//! An clip implementation to hold audio data in a vector
//mod outline;
//mod outline;
use outline::{Clip, Sample};

///A Clip Implementation that maintains audio data as a vector of Samples (see outline.rs:Sample)
///A common use for this object would be to hold the raw data, which you then reference throughout
///a project.
pub struct SampleArray {
    pub sample_rate: u32,
    pub samples: Vec<Sample>,
}


impl SampleArray {
    pub fn new(sample_rate: u32, samples: Vec<Sample>) -> Self {
        SampleArray {sample_rate, samples}
    }
}


impl Clip for SampleArray {
    /// returns the duration in samples.
    fn duration(&self) -> u64 {
        self.samples.len() as u64
    }

    /// returns the number of samples per second of this clip.
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// get the sample at a point. Returns 0 for out-of-bounds access.
    fn get(&self, sample_at: u64) -> Sample {
        if (sample_at as usize) < self.samples.len() {
            self.samples[sample_at as usize]
        } else {
            0
        }
    }
}
