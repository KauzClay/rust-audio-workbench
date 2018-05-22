mod wavreader;
//use wavreader::TimeSignature;

extern crate hound;
mod samplearray;
use hound;

fn main() {
    let samplearray = hound::WavReader::open("untitled.wav").read();
    println!("{:?}\n samples per second = {}", samplearray.samples, samplearray.samples_per_second)
}
