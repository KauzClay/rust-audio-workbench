extern crate hound;

use std::io::{Read, Write, BufReader, BufRead};
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

type RawCliCommand<W> = (fn(&mut RawCliEnvironment<W>, &str) -> Result<String, String>);

pub struct RawCliEnvironment<W: Write> {
    writer: W,
    tracks: Vec<Track>,
    clips:  Vec<Arc<Clip>>,
    commands: HashMap<String, RawCliCommand<W>>,
}

impl <W: Write> RawCliEnvironment<W> {
    pub fn new(writer: W) -> Self {
        let mut env = RawCliEnvironment {
            writer,
            tracks: Vec::new(),
            clips:  Vec::new(),
            commands: HashMap::new(),
        };
        
        env.commands.insert("copy".to_owned(), copy);
        env.commands.insert("import".to_owned(), import);
        
        env
    }
    
    fn enter_loop<R: Read>(&mut self, reader: R) {
        let mut lines = BufReader::new(reader).lines();
        while let Some(Ok(line)) = lines.next() {
            if let Some(first_word) = line.split_whitespace().next() {
                if first_word == "exit" {
                    return
                }
                let func_option = self.commands.get(first_word);
                if let Some(func) = func_option {
                    match func(self, &line) {
                        Ok(success_str) => writeln!(self.writer, "{}", success_str),   
                        Err(err_str) => writeln!(self.writer, "{}", err_str),
                    }.unwrap();
                } 
            } // else empty line, continue
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

fn copy<W: Write>(env: &mut RawCliEnvironment<W>, cmd: &str) -> Result<String, String> {
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

fn import<W: Write>(env: &mut RawCliEnvironment<W>, cmd: &str) -> Result<String, String> {
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


