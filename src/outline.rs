/// moments and durations are represented by types that implement Time.
/// Convertible to/from the equivalent number of samples. This allows time to
/// be represented in different ways, e.g. measures+beats, seconds+millis, etc.
trait Time : Sized {
    /// the number of samples that takes up the same amount of time as self 
    fn to_samples(&self, samples_per_sec: u64) -> u64;
    
    /// return a copy representing the length of time in num_samples
    fn from_samples(&self, num_samples: u64, samples_per_sec: u64) -> Self;  
}

/// 
trait Clip : Sized {
    /// returns an iterator over this clip.
    fn iter(&self) -> Iterator<Item = Self>;
    
    /// returns the duration in samples.
    fn duration(&self) -> u64;
    
    /// returns the number of samples per second of this clip.
    fn samples_per_sec(&self) -> u64;
    
    /// get the sample at a point.
    fn get(&self, sample_at: u64) -> u32;
    
    /// set the sample at a point.
    fn set(&mut self, sample_at: u64, val: u32);
    
    /// returns a subclip given a start and duration with unit of samples.
    /// used by subclip().
    fn subclip_sample(&self, start: u64, duration: u64) -> Self;
    
    // returns two subclips given the sample at which to split.
    fn split_at_sample(&self, split_at: u64) -> (Self, Self);
    
    // modifies self by concatenating with other
    fn concat(&mut self, other: &Self);
     
    /// returns a subclip given a start Time and a duration Time
    fn subclip<S: Time, T: Time>(&self, start: S, duration: T) -> Self {
        let spc = self.samples_per_sec();
        self.subclip_sample(start.to_samples(spc), duration.to_samples(spc))
    }
    
    /// returns two subclips given a Time at which to split
    fn split_at<T: Time>(&self, split_at: T) -> (Self, Self) {
        let spc = self.samples_per_sec();
        self.split_at_sample(split_at.to_samples(spc))
    }
    
    /// interpolate between samples
    /// calculates the value at (sample_at + fractional) 
    /// where 0.0 <= fractional < 1.0
    fn interpolate(&self, sample_at: u64, fractional: f64) -> u32 {
        unimplemented!()
    }
}

trait Filter {
    fn apply_sample<C: Clip>(&self, clip: &mut C, start: u64, duration: u64);
    
    fn apply<C: Clip, S: Time, T: Time>(&self, clip: &mut C, start: S, duration: T) {
        let spc = clip.samples_per_sec();
        self.apply_sample(clip, start.to_samples(spc), duration.to_samples(spc))
    }
}


