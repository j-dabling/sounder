use std::fs;
use std::path::Path;
use std::io::{Write, stdout, stdin};
use std::io::{prelude::*, BufReader};
use rodio::{Decoder, OutputStream, source::Source};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


// Checks for a valid config file, if found will create a reference table of
// sounds and activation keys that will be listened for in the input environment.
fn main() {
    let config_path = ".config";
    let mut audio_sources = Vec::new();
    if Path::new(config_path).exists() {
        println!("Config file found!\nParsing for options...");
        audio_sources = parse_config(config_path);
    }
    else {
        println!("Config file not found. A new one has been created. Please populate it.");
        create_default_config();
    }
    process_input(&audio_sources);
}

// Creates a template config file in the active directory named '.config'
fn create_default_config() {
    let mut file = fs::File::create(".config")
        .expect("Should have been able to create file, but could not.");
    file.write(b"# To add a sound effect to Sounder, add an entry in the following format:\n")
        .expect("Should have been able to write file, but could not.");
    file.write(b"# path/to/file.sound:k\n")
        .expect("Should have been able to write file, but could not.");
    file.write(b"# Where the filepath is followed by a colon, followed by the key you want to\n")
        .expect("Should have been able to write file, but could not.");
    file.write(b"# activate the sound.\n")
        .expect("Should have been able to write file, but could not.");
}

// Reads the given config file-path for media file-paths and activation keys.
// Returns a table of media file-paths with associated activation key.
fn parse_config(file_path:&str) -> Vec<Vec<String>>{
    let file = fs::File::open(file_path).expect("file not found!");
    let reader = BufReader::new(file);

    let mut source_list = Vec::new();
    for line in reader.lines() {
        let raw_line = line.unwrap().clone();
        if raw_line.clone().chars().nth(0).unwrap() != '#' {
            let source_couple: Vec<String> = raw_line.split(":")
                .map(str::to_string).collect();
            source_list.push(source_couple);
        }
    }
    return source_list;
}

// Termion code here sourced and modified from Ticki at:
// https://ticki.github.io/blog/making-terminal-applications-in-rust-with-termion/
fn process_input(audio_sources: &Vec<Vec<String>>) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}Press q to exit. Other letter keys will activate configured sound.{}",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1),
           // Hide the cursor.
           termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout, "{}{}", termion::cursor::Goto(1, 2), termion::clear::CurrentLine).unwrap();
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c)   => isolate_audio_file(&audio_sources, c),
            _              => println!("Unsupported key press."),
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

// Checks the given reference table 'audio_sources' for the file-path
// that corresponds to the given key or 'letter'.
// The matching audio file will then be played via play_audio(), or exit
// if no matching file is found.
fn isolate_audio_file(audio_sources: &Vec<Vec<String>>, letter: char) {
    for source in audio_sources {
        if source[1] == String::from(letter) {
            play_audio(&source[0]);
        }
    }
}

// Given a file-path to an audio file, will play the sound to the system output
// for three seconds.
fn play_audio(audio:&String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(fs::File::open(audio).unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples())
        .expect("Should be able to play audio file, but cannot.");

    std::thread::sleep(std::time::Duration::from_secs(3));
}