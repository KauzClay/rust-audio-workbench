//use wavreader::TimeSignature;

extern crate hound;
mod samplearray;
mod outline;
mod wavreader;
mod compounds;
use outline::{AudioReader, AudioWriter};
use std::fs;
//use samplearray::SampleArray;


fn main() {
    let samplearray = hound::WavReader::open("copy.wav").unwrap().read().unwrap();
    println!("{:?}", & samplearray.samples[0..20]);
    //println!("{:?}\n samples per second = {}", samplearray.samples, samplearray.samples_per_sec);
    let f = fs::OpenOptions::new().write(true)
                             .create_new(true)
                             .open("copy2.wav").unwrap();
    println!("{:?}", hound::WavWriter::write(f, samplearray));
}
