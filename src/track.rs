use outline::{Sample, Clip};
use compounds::{Subclip};
use std::rc::Rc;
use std::cmp::{Ordering, max};

#[derive(Clone)]
struct ClipPosition {
    clip: Rc<Clip>,
    position: u64,
}

struct TrackClip {
    samples_per_sec: u32,
    duration: u64,
    clips: Vec<ClipPosition>,
}

pub struct Track {
    name: String,
    sample_rate: u32,
    left_clips: Vec<ClipPosition>,
    right_clips: Vec<ClipPosition>,
}

//potentially make a different mono track instead of holding two of the same vectors
impl Track {
    fn new(name: String, sample_rate: u32) -> Self {
        Track {
            name,
            sample_rate,
            left_clips: Vec::new(),
            right_clips: Vec::new(),
        }
    }

    fn insert(side: &mut Vec<ClipPosition>, clip: Rc<Clip>, position: u64, sample_rate: u32) -> bool {
        if clip.samples_per_sec() != sample_rate {
            return false
        }

        let bsr = side.binary_search_by_key(&position, |cp| cp.position);
        let dur = clip.duration();

        let shift_idx = match bsr {
            Ok(idx)  => {
                side.insert(idx, ClipPosition{clip, position,});
                idx+1
            },
            Err(e) => {
                if e == 0 {
                    //do the same thing as okay, its the earliest clip
                    side.insert(e, ClipPosition{clip, position,});
                    e+1
                } else {
                    let split_idx = e - 1;
                    let cp_dur = side[split_idx].clip.duration();
                    let cp_pos = side[split_idx].position;
                    let cp_clip = side[split_idx].clip.clone();
                    let split_point = position - cp_pos;

                    if  cp_dur > split_point {
                        if let Some((l,r)) = Subclip::split(cp_clip, split_point) {
                            side[split_idx].clip = l;

                            side.insert(e, ClipPosition{clip, position,});
                            side.insert(e+1, ClipPosition{clip: r, position: position+dur});
                            e+2
                        } else {
                            //math must be wrong
                            panic!();
                        }
                    } else {
                        side.insert(e, ClipPosition{clip, position,});
                        e+1
                    }
                }
            },
        };

        for i in shift_idx..side.len() {
            side[i].position += dur;
        }

        true
    }

    pub fn insert_mono(&mut self, clip: Rc<Clip>, position: u64) {
        Self::insert(&mut self.left_clips, clip.clone(), position, self.sample_rate);
        Self::insert(&mut self.right_clips, clip, position, self.sample_rate);
    }

    fn insert_stereo() {
        unimplemented!()
    }

    fn remove(side: &mut Vec<ClipPosition>, position: u64, duration: u64) {
        unimplemented!()
    }

    fn duration(&self) -> u64 {
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

    fn track_to_clip(&self, side: &Vec<ClipPosition>) -> Rc<Clip> {
        let tc = TrackClip{
            samples_per_sec: self.sample_rate,
            duration: self.duration(),
            clips: side.clone(),
        };

        Rc::new(tc)

    }

}

impl Clip for TrackClip {
    fn duration(&self) -> u64 {
        self.duration
    }

    /// returns the number of samples per second of this clip.
    fn samples_per_sec(&self) -> u32 {
        self.samples_per_sec
    }

    /// get the sample at a point.
    fn get(&self, sample_at: u64) -> Sample {
        let bsr = self.clips.binary_search_by_key(&sample_at, |cp| cp.position);
        match bsr {
            Ok(idx) => {
                return self.clips[idx].clip.get(0)
            },
            Err(e)  => {
                if e == 0 {
                    return 0
                } else {
                    let get_idx = e - 1;
                    let cp_dur = self.clips[get_idx].clip.duration();
                    let cp_pos = self.clips[get_idx].position;
                    let get_point = sample_at - cp_pos;

                    return self.clips[get_idx].clip.get(get_point)

                }
            }
        }
    }
}
