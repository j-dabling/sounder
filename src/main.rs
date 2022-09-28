//use std::env;
use std::fs;
use std::path::Path;
use std::io::{Read, Write, Cursor};
use std::io::{self, prelude::*, BufReader};
use rodio::{Decoder, OutputStream, source::Source};

//===========================================================================//

struct Player {
    media_source: String
}

impl Player {
    fn testeroonie(&self, audio:&str) {
        println!("Sourcing audio files from folder: {}", self.media_source);
        // Get a output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(fs::File::open(audio).unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        // Play the sound directly on the device
        stream_handle.play_raw(source.convert_samples());

        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}

//===========================================================================//

fn main() {
    let config_path = ".config";
    if Path::new(config_path).exists() {
        println!("Config file found!\nParsing for options...");
        parse_config(".config");
        let test_player = Player{media_source: String::from("media")};
        //test_player.testeroonie("media/test.mp3");
    }
    else {
        println!("Config file not found. Creating it now.");
        create_default_config();
    }
    //read_poem()
}

// Reads a demo text file, based off of the Rust Doc tutorial.
fn read_poem() {
    let file_path = "../poem.txt";
    println!("From the file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Cannot read file at given path.");

    println!("{contents}");
}

// Checks to see if the default config file exists.
// If no file is found, one will be created and populated by default.
fn create_default_config() {
    let mut file = fs::File::create(".config")
        .expect("Should have been able to create file.");

    file.write(b"");
    // file.write(b"CONFIG_PATH: .config\n")
    //     .expect("Should have been able to write to file.");
    
    // file.write(b"MEDIA_PATH: media/\n")
    //     .expect("Should have been able to write to file.");
}

// Sets environment variables according to the options found in config file.
fn parse_config(file_path:&str) {
    let file = fs::File::open(file_path).expect("file not found!");
    let reader = BufReader::new(file);

    let mut source_list: Vec<Vec<String>> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        println!("{:?}", line.split(':')).collect();
        // let source_couple: Vec<String> = line.split(':');
        // println!("{:?}", source_couple);
        // source_list.push(source_couple);



        // println!("{:?}", source_list);
        // //println!("{:?}", line.rfind(':'));
        // let seperator = line.rfind(':');
        // // println!("{}", seperator)
        // if (seperator > Some(0)) {
        //     println!("HELLO HELLO HELLO HELLO{:?}", seperator);
        // }
        // else {
        //     println!("There is no config option to specify.")
        // }
    }
}

//===========================================================================//

