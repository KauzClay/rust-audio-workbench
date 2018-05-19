use std::fmt;


#[derive(Debug)]
pub struct TimeSignature {
    beats: u16,     // example: 3 time
    beat_unit: u16, //          8
}


#[derive(Debug)]
pub struct Tempo {
    time_signature: TimeSignature,
    bpm: f64,
}


impl TimeSignature {
    pub fn new(beats: u16, beat_unit: u16) -> Self {
        TimeSignature { beats, beat_unit }
    }
    
    pub fn common_time() -> Self { Self::new(4, 4) }
    pub fn cut_time() -> Self { Self::new(2, 2) }
}


impl fmt::Display for TimeSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.beats, self.beat_unit)
    }
}


impl Tempo {
    pub fn new(time_signature: TimeSignature, bpm: f64) -> Self {
        Tempo { time_signature, bpm }
    }
    
    pub fn beat_length_sec(&self) -> f64 {
        60.0 / self.bpm
    }
    
    pub fn measure_length_sec(&self) -> f64 {
        self.beat_length_sec() * self.time_signature.beats as f64
    }
}


impl fmt::Display for Tempo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} bpm ({})", self.bpm, self.time_signature)
    }
}

