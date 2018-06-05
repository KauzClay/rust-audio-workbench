//! The core trait for implementing structs that represent audio data

use std::io;
use std::io::{Read, Write, Seek};
use std::sync::Arc;
use samplearray::SampleArray;

///A type to represent the size of one unit of audio data
pub type Sample = i16;

/// moments and durations are represented by types that implement Time.
/// Convertible to/from the equivalent number of samples. This allows time to
/// be represented in different ways, e.g. measures+beats, seconds+millis, etc.
pub trait Time : Sized {
    /// the number of samples that takes up the same amount of time as self
    fn to_samples(&self, sample_rate: u32) -> u64;

    /// return a copy representing the length of time in num_samples
    fn from_samples(&self, num_samples: u64, sample_rate: u32) -> Self;
}

impl Time for u64 {
    /// u64's are treated like the raw sample count, so calling to_samples
    /// on a u64 returns self.
    fn to_samples(&self, _sample_rate: u32) -> u64 {
        *self
    }

    /// u64's are treated like the raw sample count, so calling from_samples
    /// on a u64 returns self.
    fn from_samples(&self, num_samples: u64, _sample_rate: u32) -> Self {
        num_samples
    }
}

impl Time for f64 {
    /// f64's are treated as a real valued duration in seconds.
    fn to_samples(&self, sample_rate: u32) -> u64 {
        (self * sample_rate as f64) as u64
    }

    /// u64's are treated like the raw sample count, so calling from_samples
    /// on a u64 returns self.
    fn from_samples(&self, num_samples: u64, sample_rate: u32) -> Self {
        num_samples as f64 / sample_rate as f64
    }
}

///The core functionality required by any struct that is to hold/represent audio data
pub trait Clip {
    //from_iter()? or some other way to mix several Clips, not sure
    // where that code should go

    /// returns an iterator over this clip.
    //fn iter(&self) -> Iterator<Item = Self>;

    /// returns the duration of this clip in number of samples.
    fn duration(&self) -> u64;

    /// returns the number of samples per second of this clip.
    fn sample_rate(&self) -> u32;

    /// get the sample at a point. Returns 0 if out of range.
    fn get(&self, sample_at: u64) -> Sample;

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

///Any filter-style modification of audio data can be achieved by implementing structs
///following this trait
pub trait Filter {
    fn apply_sample<C: Clip>(&self, clip: &mut C, start: u64, duration: u64);

    fn apply<C: Clip, S: Time, T: Time>(&self, clip: &mut C, start: S, duration: T) {
        let spc = clip.sample_rate();
        self.apply_sample(clip, start.to_samples(spc), duration.to_samples(spc))
    }
}

///A trait for reading various audio files into compliant structures for manipulation in raw
pub trait AudioReader {
    type Reader: Read;

    /// Converts an n-channel audio file into a vector of n reference-counted
    /// SampleArrays. Returns None upon failure.
    fn read(&mut self) -> Option<Vec<Arc<SampleArray>>>;
}

///A trait for writing raw clips into various audio files
pub trait AudioWriter {
    type Writer: Write + Seek;

    ///Writes a Clip to a specified audio file
    //TODO allow for n-channel writes
    fn write(w: Self::Writer, c: &Clip) -> bool;
}
