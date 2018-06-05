use std::sync::Arc;
use outline::{Clip, Time, Sample};

///A struct used to represent a subset of audio data within a larger set via reference.
///Good for working with smaller chunks of data out of a SampleArray.
pub struct Subclip {
    clip: Arc<Clip>,
    start: u64,
    duration: u64,
}


impl Subclip {
    ///The split time (start_time + duration_time) must be within the duration of the clip,
    ///else None is returned.
    pub fn new<S: Time, T: Time>(clip: Arc<Clip>, start_time: S, duration_time: T) -> Option<Arc<Self>> {
        let spc = clip.sample_rate();
        let start = start_time.to_samples(spc);
        let duration = duration_time.to_samples(spc);
        
        if clip.duration() < start + duration {
            None
        } else {
            Some(Arc::new(Subclip {clip, start, duration}))
        }
    }

    ///Returns a SubClip from the beginning of the source clip to a specified time into the clip.
    pub fn from_start<T: Time>(clip: Arc<Clip>, duration_time: T) -> Option<Arc<Self>> {
        Self::new(clip, 0u64, duration_time)
    }

    ///Returns a SubClip from the specified start time to the end of the source clip.
    pub fn to_end<T: Time>(clip: Arc<Clip>, start_time: T) -> Option<Arc<Self>> {
        let spc = clip.sample_rate();
        let start = start_time.to_samples(spc);
        let duration = clip.duration();
        Self::new(clip, start, duration - start)
    }

    ///Returns two SubClips, one from the beginning of the source clip to the split point,
    ///and one from the split point to the end of the source clip.
    pub fn split<T: Time>(clip: Arc<Clip>, split_time: T) -> Option<(Arc<Self>, Arc<Self>)> {
        let spc = clip.sample_rate();
        let split = split_time.to_samples(spc);
        let clip2 = clip.clone();
        if let Some(first_half) = Self::from_start(clip, split) {
            if let Some(second_half) = Self::to_end(clip2, split) {
                return Some((first_half, second_half));
            }
        }
        None
    }
}


impl Clip for Subclip {
    //from_iter()? or some other way to mix several Clips, not sure
    // where that code should go

    /// returns an iterator over this clip.
    //fn iter(&self) -> Iterator<Item = Self>;

    /// returns the duration in samples.
    fn duration(&self) -> u64 {
        self.duration
    }

    /// returns the number of samples per second of this clip.
    fn sample_rate(&self) -> u32 {
        self.clip.sample_rate()
    }

    /// get the sample at a point.
    fn get(&self, sample_at: u64) -> Sample {
        self.clip.get(self.start + sample_at)
    }
}


///A struct used to represent the joining of two Clip objects.
///Good for chaining together SubClips.
pub struct Concat {
    left: Arc<Clip>,
    right: Arc<Clip>,
    left_duration: u64,
    total_duration: u64,
}


impl Concat {
    ///The two clips to be joined must use the same sample rate, or else None is returned.
    pub fn new(left: Arc<Clip>, right: Arc<Clip>) -> Option<Arc<Self>> {
        if left.sample_rate() == right.sample_rate() {
            let left_duration = left.duration();
            let total_duration = left_duration + right.duration();
            Some(Arc::new(Concat {left, right, left_duration, total_duration}))
        } else {
            None
        }
    }
}


impl Clip for Concat {
    //from_iter()? or some other way to mix several Clips, not sure
    // where that code should go

    /// returns an iterator over this clip.
    //fn iter(&self) -> Iterator<Item = Self>;

    /// returns the duration in samples.
    fn duration(&self) -> u64 {
        self.total_duration
    }

    /// returns the number of samples per second of this clip.
    fn sample_rate(&self) -> u32 {
        self.left.sample_rate()
    }

    /// get the sample at a point.
    fn get(&self, sample_at: u64) -> Sample {
        if sample_at > self.left_duration {
            self.right.get(sample_at - self.left_duration)
        } else {
            self.left.get(sample_at)
        }

    }
}

///A simple struct to represent the reverse of a clip.
pub struct Reverse(Arc<Clip>);

impl Reverse {
    pub fn new(clip: Arc<Clip>) -> Self {
        Reverse(clip)
    }
}

impl Clip for Reverse {
    /// returns the duration in samples.
    fn duration(&self) -> u64 {
        self.0.duration()
    }

    /// returns the number of samples per second of this clip.
    fn sample_rate(&self) -> u32 {
        self.0.sample_rate()
    }

    /// get the sample at a point. Samples are 'got' by inverting the sample_at index.
    ///EX: sample_at of 0 returns Sample N, sample_at of 1 returns Sample N-1, etc.
    fn get(&self, sample_at: u64) -> Sample {
        self.0.get(self.0.duration() - sample_at - 1)
    }
}
