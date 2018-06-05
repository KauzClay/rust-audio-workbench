extern crate hound;

use std::io::{Read, Write};
use std::collections::HashMap;
use std::sync::Arc;
use std::ops::Fn;
use std::path::Path;
use std::ffi::OsStr;

use wavreader;
use track::Track;
use outline::{Clip, AudioReader};
use samplearray::SampleArray;

type RawCliCommand<R, W> = (fn(&RawCliEnvironment<R, W>, &str) -> Result<String, String>);

struct RawCliEnvironment<R: Read, W: Write> {
    reader: R,
    writer: W,
    tracks: Vec<Track>,
    clips:  Vec<Arc<Clip>>,
    commands: HashMap<String, RawCliCommand<R, W>>,
}

fn copy<R: Read, W: Write>(env: &mut RawCliEnvironment<R, W>, cmd: &str) -> Result<String, String> {
    let mut words = cmd.split_whitespace();
    
    // copy <track name> <start time> <duration> <clip name>
    //
    if words.next() != Some("copy") {
        return Err(format!("Unknown internal error"));
    }
    
    let trackname = if let Some(word) = words.next() {
        word
    } else {
        return Err(format!("Not enough arguments. Syntax is: 'copy <track name> <start time> <duration> <clip name>'"));
    };
    
    let track = if let Some(index) = env.tracks.iter().position(|ref t| t.name() == trackname) {
        &env.tracks[index]
    } else {
        return Err(format!("Track with name {} not found.", trackname));
    };
    
    let start = if let Some(Ok(s)) = words.next().map(|w| w.parse::<f64>()) {
        s
    } else {
        return Err(format!("Syntax is: 'copy <track name> <start time> <duration> <clip name>'"));
    };
    
    let duration = if let Some(Ok(d)) = words.next().map(|w| w.parse::<f64>()) {
        d
    } else {
        return Err(format!("Syntax is: 'copy <track name> <start time> <duration> <clip name>'"));
    };
    
    let clip_name = if let Some(word) = words.next() {
        word
    } else {
        return Err(format!("Not enough arguments. Syntax is: 'copy <track name> <start time> <duration> <clip name>'"));
    };
    
    //let mut samples = Vec::new();
    
    Ok(format!(""))
}

fn import<R: Read, W: Write>(env: &mut RawCliEnvironment<R, W>, cmd: &str) -> Result<String, String> {
    let mut words = cmd.split_whitespace();
    
    if words.next() != Some("import") {
        return Err(format!("Unknown internal error"));
    }
    
    let filename = if let Some(word) = words.next() {
        word
    } else {
        return Err(format!("Not enough arguments. Syntax is: 'import <file name> <track name>'"));
    };
    
    let trackname = if let Some(word) = words.next() {
        word
    } else {
        return Err(format!("Not enough arguments. Syntax is: 'import <file name> <track name>'"));
    };
    
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


