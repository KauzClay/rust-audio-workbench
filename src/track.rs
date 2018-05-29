use outline::Clip;
use std::rc::Rc;

pub struct ClipPosition {
    clip: Rc<Clip>,
    position: u64,
}

pub struct Track {
    name: String,
    left_clips: Vec<ClipPosition>,
    right_clips: Vec<ClipPosition>,
}

