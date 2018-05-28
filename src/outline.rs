use std::io;
use std::io::{Read, Write, Seek};
use std::rc::Rc;
use samplearray::SampleArray;

pub type Sample = i16;
/// moments and durations are represented by types that implement Time.
/// Convertible to/from the equivalent number of samples. This allows time to
/// be represented in different ways, e.g. measures+beats, seconds+millis, etc.
pub trait Time : Sized {
    /// the number of samples that takes up the same amount of time as self
    fn to_samples(&self, samples_per_sec: u32) -> u64;

    /// return a copy representing the length of time in num_samples
    fn from_samples(&self, num_samples: u64, samples_per_sec: u32) -> Self;
}

impl Time for u64 {
    fn to_samples(&self, _samples_per_sec: u32) -> u64 {
        *self
    }
    
    fn from_samples(&self, num_samples: u64, _samples_per_sec: u32) -> Self {
        num_samples
    }
}

///
pub trait Clip {
    //from_iter()? or some other way to mix several Clips, not sure
    // where that code should go

    /// returns an iterator over this clip.
    //fn iter(&self) -> Iterator<Item = Self>;

    /// returns the duration in samples.
    fn duration(&self) -> u64;

    /// returns the number of samples per second of this clip.
    fn samples_per_sec(&self) -> u32;

    /// get the sample at a point.
    fn get(&self, sample_at: u64) -> Sample;
    // should this panic or return a Result instead upon out-of-bounds access
    // consider performance

    /// set the sample at a point.
    ///fn set(&mut self, sample_at: u64, val: Sample);
/*
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
    }*/

    /// interpolate between samples
    /// calculates the value at (sample_at + fractional)
    /// where 0.0 <= fractional < 1.0
    /// the current interpolation function is linear; this should definitely
    /// use a better method eventually.
    fn interpolate(&self, sample_at: u64, fractional: f64) -> Option<Sample> {
        assert!(fractional >= 0.0 && fractional < 1.0);
        let dur = self.duration();
        if sample_at <= 0 || sample_at >= dur {
            // out of range
            None
        } else if sample_at == dur - 1 {
            // within range, but only one sample to "interpolate" from, so just
            // return that value
            Some(self.get(sample_at))
        } else {
            // y = mx + b
            // the two points to interpolate between are (0, y0) and (1, y1)
            // so m = y1 - b and b = y0.
            let b = self.get(sample_at + 1) as f64;
            let m = self.get(sample_at) as f64 - b;

            Some((m * fractional + b).round() as Sample)
        }
    }
}

pub trait Filter {
    fn apply_sample<C: Clip>(&self, clip: &mut C, start: u64, duration: u64);

    fn apply<C: Clip, S: Time, T: Time>(&self, clip: &mut C, start: S, duration: T) {
        let spc = clip.samples_per_sec();
        self.apply_sample(clip, start.to_samples(spc), duration.to_samples(spc))
    }
}

pub trait AudioReader {
    type Reader: Read;
    fn read(&mut self) -> Option<Rc<SampleArray>>;
}

pub trait AudioWriter {
    type Writer: Write + Seek;
    fn write(w: Self::Writer, c: Rc<Clip>) -> bool;
}
