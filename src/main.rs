//use wavreader::TimeSignature;

extern crate hound;
mod samplearray;
mod outline;
mod wavreader;
mod compounds;
mod track;
mod rawcli;
use std::io::{stdin, stdout};
use outline::{Clip, AudioReader, AudioWriter};
use samplearray::SampleArray;
use compounds::{Concat, Subclip, Reverse};
use std::fs;
use rawcli::RawCliEnvironment;
//use samplearray::SampleArray;


fn main() {
    let mut env = RawCliEnvironment::new();
    env.enter_loop(stdin(), stdout());
    
    //let channels = hound::WavReader::open("untitled.wav").unwrap().read().unwrap();
    //let samplearray = channels[0].clone();
    //let half = samplearray.duration() / 2;
    //if let Some((left, right)) = Subclip::split(samplearray, half) {
    //    let left = pair.0;
    //    let right = pair.1;
    //    let rev = Reverse::new(samplearray);
    //    let concat = Concat::new(right, left).expect("concat failed");
    //    let f = fs::OpenOptions::new().write(true)
    //                     .create(true)
    //                     .open("copy.wav").unwrap();
    
    //current limitation is that AudioWriter trait only handles mono, so for
    //demonstration purposes we're only working with one channel rn
    
    //        println!("{:?}", hound::WavWriter::write(f, &rev));
    //} else {
    //    panic!("split failed");
    //}
}
