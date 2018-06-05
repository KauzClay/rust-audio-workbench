//! Track struct that is used to maintain various clips assosiated with each other

use outline::{Sample, Clip, Time};
use compounds::Subclip;
use std::sync::Arc;
use std::cmp::max;

#[derive(Clone)]
struct ClipPosition {
    clip: Arc<Clip>,
    position: u64,
}

struct TrackClip {
    sample_rate: u32,
    duration: u64,
    clips: Vec<ClipPosition>,
}

///A struct to represent a track containing a left and right channel.
///In future iterations, Track will be able to handle N channels.
pub struct Track {
    name: String,
    sample_rate: u32,
    left_clips: Vec<ClipPosition>,
    right_clips: Vec<ClipPosition>,
}

//potentially make a different mono track instead of holding two of the same vectors
impl Track {
    /// Creates a new, empty clip with a given name and sample rate.
    ///
    /// # Example
    /// let track = Track::new("my_track".to_owned(), 44100);
    ///
    pub fn new(name: String, sample_rate: u32) -> Self {
        Track {
            name,
            sample_rate,
            left_clips: Vec::new(),
            right_clips: Vec::new(),
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }
    // inserts a clip into a vector compatible with Track, where side is either
    // the left or the right Vec.
    // TODO refactor
    fn insert(side: &mut Vec<ClipPosition>, clip: Arc<Clip>, position: u64, dur: u64) -> bool {
        let bsr = side.binary_search_by_key(&position, |cp| cp.position);
        //let dur = duration;//clip.duration();
        if dur < clip.duration() {
            return false
        }

        let shift_index = match bsr {
            Ok(index)  => {
                side.insert(index, ClipPosition{clip, position,});
                index + 1 // the index of the first clip to shift
            },
            Err(index) => {
                if index == 0 {
                    //there are no clips before the insertion point; so no clip
                    //needs to be split, so just insert
                    side.insert(index, ClipPosition{clip, position,});
                    index + 1 // the index of the first clip to shift
                } else {
                    // the previous clip needs to be split if the previous
                    // clip overlaps with the position of the new clip
                    let split_index = index - 1;
                    let split_duration = side[split_index].clip.duration();
                    let split_position = side[split_index].position;
                    let split_clip = side[split_index].clip.clone();
                    let split_point = position - split_position;

                    if  split_duration > split_point {
                        if let Some((l, r)) = Subclip::split(split_clip, split_point) {
                            side[split_index].clip = l;

                            side.insert(index, ClipPosition{clip, position,});
                            side.insert(index + 1, ClipPosition{clip: r, position: position + dur});
                            index + 2 // the index of the first clip to shift
                        } else {
                            //math must be wrong
                            panic!("Rust Audio Workbench nonrecoverable error");
                        }
                    } else {
                        side.insert(index, ClipPosition{clip, position,});
                        index + 1 // the index of the first clip to shift
                    }
                }
            },
        };

        // shift each later clip over by the inserted clip's duration
        for i in shift_index..side.len() {
            side[i].position += dur;
        }

        true
    }

    /// Inserts a Clip into self, shifting later clips by the inserted clip's duration.
    ///
    pub fn insert_mono<T: Time>(&mut self, clip: Arc<Clip>, position_time: T) -> bool {
        let position = position_time.to_samples(self.sample_rate());
        if clip.sample_rate() != self.sample_rate {
            false
        } else {
            let dur = clip.duration();
            if !Self::insert(&mut self.left_clips, clip.clone(), position, dur) {
                return false
            }
            if !Self::insert(&mut self.right_clips, clip, position, dur) {
                return false
            }

            true
        }
    }

    pub fn insert_stereo(&mut self, left_clip: Arc<Clip>, right_clip: Arc<Clip>, position: u64) -> bool {
        if left_clip.sample_rate() != self.sample_rate || right_clip.sample_rate() != self.sample_rate{
            false
        } else {

            let l_dur = left_clip.duration();
            let r_dur = right_clip.duration();
            let dur = max(l_dur, r_dur);
            if !Self::insert(&mut self.left_clips, left_clip, position, dur) {
                return false
            }
            if !Self::insert(&mut self.right_clips, right_clip, position, dur) {
                return false
            }

            true
        }
    }

    fn remove(side: &mut Vec<ClipPosition>, position: u64, duration: u64) {
        unimplemented!()
    }

    /// Returns the duration of the entire Track. If the mono and stero channels
    /// have different durations, the longer one is returned.
    pub fn duration(&self) -> u64 {
        let left_dur = match self.left_clips.iter().last() {
            Some(cp) => cp.position + cp.clip.duration(),
            None     => 0,
        };

        let right_dur = match self.right_clips.iter().last() {
            Some(cp) => cp.position + cp.clip.duration(),
            None     => 0,
        };

        max(right_dur, left_dur)
    }

    pub fn left_channel_as_clip(&self) -> Arc<Clip> {
        self.track_to_clip(&self.left_clips)
    }

    pub fn right_channel_as_clip(&self) -> Arc<Clip> {
        self.track_to_clip(&self.right_clips)
    }

    // convert a channel of a track into a clip
    fn track_to_clip(&self, side: &Vec<ClipPosition>) -> Arc<Clip> {
        let tc = TrackClip{
            sample_rate: self.sample_rate,
            duration: self.duration(),
            clips: side.clone(),
        };

        Arc::new(tc)
    }
}

impl Clip for TrackClip {
    fn duration(&self) -> u64 {
        self.duration
    }

    /// returns the number of samples per second of this clip.
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// get the sample at a point.
    fn get(&self, sample_at: u64) -> Sample {
        let bsr = self.clips.binary_search_by_key(&sample_at, |cp| cp.position);
        match bsr {
            Ok(index) => {
                self.clips[index].clip.get(0)
            },
            Err(index)  => {
                if index == 0 {
                    0
                } else {
                    let get_idx = index - 1;
                    let split_dur = self.clips[get_idx].clip.duration();
                    let split_pos = self.clips[get_idx].position;
                    let get_point = sample_at - split_pos;
                    self.clips[get_idx].clip.get(get_point)
                }
            }
        }
    }
}
