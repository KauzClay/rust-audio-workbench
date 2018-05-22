//mod outline;
//mod outline;
use outline::Clip;

pub struct SampleArray {
    pub samples_per_sec: u64,
    pub samples: Vec<i32>,
}


impl SampleArray {
    fn new(samples_per_sec: u64, samples: Vec<i32>) -> Self {
        SampleArray {samples_per_sec, samples}
    }
}


impl Clip for SampleArray {
    
    /// returns the duration in samples.
    fn duration(&self) -> u64 {
        self.samples.len() as u64
    }
    
    /// returns the number of samples per second of this clip.
    fn samples_per_sec(&self) -> u64 {
        self.samples_per_sec
    }
    
    /// get the sample at a point.
    fn get(&self, sample_at: u64) -> i32 {
        self.samples[sample_at as usize]
    }
    // should this panic or return a Result instead upon out-of-bounds access
    // consider performance
    
    /// set the sample at a point.
    fn set(&mut self, sample_at: u64, val: i32) {
        self.samples[sample_at as usize] = val;
    }
    
    /// returns a subclip given a start and duration with unit of samples.
    /// used by subclip().
    fn subclip_sample(&self, start: u64, duration: u64) -> Self {
        unimplemented!();
    }
    
    // returns two subclips given the sample at which to split.
    fn split_at_sample(&self, split_at: u64) -> (Self, Self) {
        unimplemented!();
    }
    
    // modifies self by concatenating with other
    fn concat(&mut self, other: &Self) {
        unimplemented!();
    }
}
