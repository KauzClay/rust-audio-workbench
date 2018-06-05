extern crate hound;

use std::io::{Read, Write};
use std::collections::HashMap;
use std::sync::Arc;
use std::ops::Fn;
use std::path::Path;
use std::ffi::OsStr;
use std::str::SplitWhitespace;

use wavreader;
use track::Track;
use outline::{Clip, AudioReader};
use compounds::Subclip;
use samplearray::SampleArray;

type RawCliCommand<R, W> = (fn(&RawCliEnvironment<R, W>, &str) -> Result<String, String>);

pub struct RawCliEnvironment<R: Read, W: Write> {
    reader: R,
    writer: W,
    tracks: Vec<Track>,
    clips:  Vec<Arc<Clip>>,
    commands: HashMap<String, RawCliCommand<R, W>>,
}

impl <R: Read, W: Write> RawCliEnvironment<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        RawCliEnvironment {
            reader,
            writer,
            tracks: Vec::new(),
            clips:  Vec::new(),
            commands: HashMap::new()
        }
    }
    
}

fn check_num_args<'a>(cmd: &'a str, num: usize, syntax: &str) -> Result<SplitWhitespace<'a>, String> {
    // num does not include first word in command; split_whitespace.len does
    // do not subtract because numerical underflow is panic
    let num_args_plus_one = cmd.split_whitespace().collect::<Vec<&str>>().len();
    if  num_args_plus_one != num + 1 {
        Err(format!("Expected {} arguments, found {}. Syntax is: '{}'", num, num_args_plus_one as i64 - 1, syntax))
    } else {
        Ok(cmd.split_whitespace())
    }
}

fn check_keyword(found: Option<&str>, expected: &str) -> Result<(), String> {
    if found == None {
        Err(format!("Expected keyword {}", expected))
    } else if found != Some(expected) {
        Err(format!("Expected keyword {}, found {}", expected, found.unwrap()))
    } else {
        Ok(())
    }
}

fn parse_f64(word: &str) -> Result<f64, String> {
    word.parse::<f64>().map_err(|_| format!("Expected number, found {}", word))
}

fn copy<R: Read, W: Write>(env: &mut RawCliEnvironment<R, W>, cmd: &str) -> Result<String, String> {
    let mut words = check_num_args(cmd, 3, "copy <track name> <start time> <duration>")?;
    
    check_keyword(words.next(), "copy")?;
    
    // already checked number of args; can unwrap
    let trackname = words.next().unwrap();

    let track = if let Some(index) = env.tracks.iter().position(|ref t| t.name() == trackname) {
        &env.tracks[index]
    } else {
        return Err(format!("Track with name {} not found.", trackname));
    };
    
    let start = parse_f64(words.next().unwrap())?;
    let duration = parse_f64(words.next().unwrap())?;
    
    let left = Subclip::new(track.left_channel_as_clip(), start, duration)
        .ok_or_else(|| format!("Start or duration out of bounds"))?;
    let right = Subclip::new(track.right_channel_as_clip(), start, duration)
        .ok_or_else(|| format!("Start or duration out of bounds"))?;
    
    let left_index = env.clips.len();
    env.clips.push(left);
    env.clips.push(right);
    
    Ok(format!("Left copied to Clip {}, right copied to Clip {}", left_index, left_index + 1))
}

fn import<R: Read, W: Write>(env: &mut RawCliEnvironment<R, W>, cmd: &str) -> Result<String, String> {
    let mut words = check_num_args(cmd, 2, "import <file name> <track name>")?;
    check_keyword(words.next(), "import")?;
    let filename = words.next().unwrap();
    let trackname =  words.next().unwrap();
    let path = Path::new(filename);
    
    if path.extension() == Some(OsStr::new("wav")) {
        if let Ok(mut reader) = hound::WavReader::open(path) {
            let sample_rate = reader.spec().sample_rate;
            if let Some(vec) = reader.read() {
                
                let mut track = Track::new(trackname.to_string(), sample_rate);
                if vec.len() == 1 {
                    track.insert_mono(vec[0].clone(), 0);
                    env.tracks.push(track);
                    Ok(format!("Created track {}", trackname))
                } else if vec.len() == 2 {
                    //track.insert_stereo(vec[0].clone(), vec[1].clone(), 0);
                    env.tracks.push(track);
                    Ok(format!("Created track {}", trackname))
                } else if vec.len() > 2 {
                    //track.insert_stereo(vec[0].clone(), vec[1].clone(), 0);
                    env.tracks.push(track);
                    Ok(format!(
                        "Warning: {} contains more than 2 channels; channels beyond the first two were truncated.", 
                        filename))
                } else { // == 0
                    env.tracks.push(track);
                    Ok(format!("File was empty; created empty track {}", trackname))
                }
            } else {
                Err(format!("Error reading from file {}.", filename))
            }
        } else {
            Err(format!("Error reading from file {}", filename))
        }
    } else {
        if let Some(Some(filetype)) = path.extension().map(|s| s.to_str()) {
            Err(format!("Unsupported file type {}.", filetype))
        } else {
            Err(format!("Unsupported file type ?."))
        }
    }
}


