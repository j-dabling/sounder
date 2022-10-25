use std::fs;
use std::path::Path;
use std::io::{Write, stdout, stdin};
use std::io::{prelude::*, BufReader};
use rodio::{Decoder, OutputStream, source::Source};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


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

// Checks to see if the default config file exists.
// If no file is found, one will be created and populated by default.
fn create_default_config() {
    let mut file = fs::File::create(".config")
        .expect("Should have been able to create file, but could not.");
    //let config_template = fs:: File::open(Path::new("src/default_config"))
        //.expect("Should have been able to open `default_config`.");
    file.write(b"# To add a sound effect to Sounder, add an entry in the following format:\n")
        .expect("Should have been able to write file, but could not.");
    file.write(b"# path/to/file.sound:k\n")
        .expect("Should have been able to write file, but could not.");
    file.write(b"# Where the filepath is followed by a colon, followed by the key you want to\n")
        .expect("Should have been able to write file, but could not.");
    file.write(b"# activate the sound.\n")
        .expect("Should have been able to write file, but could not.");
}

// Sets environment variables according to the options found in config file.
fn parse_config(file_path:&str) -> Vec<Vec<String>>{
    let file = fs::File::open(file_path).expect("file not found!");
    let reader = BufReader::new(file);

    // Creates a list to keep track of all media files and their hot-keys.
    let mut source_list = Vec::new();
    for line in reader.lines() {
        // Unwraps and splits the str into the file and the key strings,
        // then maps each str into a completely new String vector to avoid complications
        // with the lifetime of 'line'.
        let raw_line = line.unwrap().clone();
        if raw_line.clone().chars().nth(0).unwrap() != '#' {
            let source_couple: Vec<String> = raw_line.split(":")
                .map(str::to_string).collect();
            // Pushes the new vector out into 'source_list'.
            source_list.push(source_couple);
        }
    }
    return source_list;
}

// Termion code here sourced and modified from Ticki at:
// https://ticki.github.io/blog/making-terminal-applications-in-rust-with-termion/
fn process_input(audio_sources: &Vec<Vec<String>>) {
    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}Press q to exit. Other letter keys will activate configured sound.{}",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1),
           // Hide the cursor.
           termion::cursor::Hide).unwrap();
    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();

    for c in stdin.keys() {
        // Clear the current line.
        write!(stdout, "{}{}", termion::cursor::Goto(1, 2), termion::clear::CurrentLine).unwrap();

        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Char('q') => break,
            Key::Char(c)   => isolate_audio_file(&audio_sources, c),
            _              => println!("Unsupported key press."),
        }

        // Flush again.
        stdout.flush().unwrap();
    }

    // Make the cursor visible again.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn isolate_audio_file(audio_sources: &Vec<Vec<String>>, letter: char) {
    for source in audio_sources {
        if source[1] == String::from(letter) {
            play_audio(&source[0]);
        }
    }
}

fn play_audio(audio:&String) {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(fs::File::open(audio).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples())
        .expect("Should be able to play audio file, but cannot.");

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(3));
}