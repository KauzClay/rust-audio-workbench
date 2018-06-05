//use wavreader::TimeSignature;

extern crate hound;
mod samplearray;
mod outline;
mod wavreader;
mod compounds;
mod track;
mod rawcli;
use outline::{Clip, AudioReader, AudioWriter};
use samplearray::SampleArray;
use compounds::{Concat, Subclip, Reverse};
use std::fs;
//use samplearray::SampleArray;


fn main() {
    let channels = hound::WavReader::open("untitled.wav").unwrap().read().unwrap();
    let samplearray = channels[0].clone();
    
    //let half = samplearray.duration() / 2;
    //println!("{:?}", & samplearray.samples[0..20]);
    //println!("{:?}\n samples per second = {}", samplearray.samples, samplearray.sample_rate);
    
    
    //if let Some((left, right)) = Subclip::split(samplearray, half) {
    //let left = pair.0;
    //let right = pair.1;
    let rev = Reverse::new(samplearray);
    //    let concat = Concat::new(right, left).expect("concat failed");
    let f = fs::OpenOptions::new().write(true)
                         .create(true)
                         .open("copy.wav").unwrap();
    // current limitation is that AudioWriter trait only handles mono, so for
    // demonstration purposes we're only working with one channel rn
    println!("{:?}", hound::WavWriter::write(f, &rev));
    //} else {
    //    panic!("split failed");
    //}
    
    
    
    
    
    
}
