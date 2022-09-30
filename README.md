# Sounder

A simple CLI-operated soundboard written using Rust, Rodio, and Termion. Sounder can be launched from the terminal, then pressing pre-defined keys will trigger an associated sound to play over the system's enabled audio output. 

Audio Files (.wav, .mp3) can be paired to keys in the .config file with the following syntax:
```
path/to/file.wav:k
```
Where the file path is followed by a colon ':' then followed by whichever key you choose to activate it, in this case, 'k'. Keep in mind that the key mappings are case sensitive. In the event of two or more files having the same key assignment, the first one listed will be the one played.

This project was a good chance to break into Rust, and to expirement with using audio and external crates.

{Provide a link to your YouTube demonstration.  It should be a 4-5 minute demo of the software running and a walkthrough of the code.  Focus should be on sharing what you learned about the language syntax.}

[Video Demo](https://youtu.be/UiKCjMzMM_w)

# Development Environment

* Cargo 1.63.0
* Rust 1.63.0
* Termion (For accepting input)
* Rodio (Audio playback)

# Useful Websites

* [Official Rust Documentation](https://doc.rust-lang.org/stable/book/)
* [Difference between &str and String in Rust](https://blog.mgattozzi.dev/how-do-i-str-string/)
* [Scope management](https://stackoverflow.com/questions/24689463/collect-into-owned-vec-of-owned-strings-in-rust)

# Future Work

* Get absolute paths working.
* Dynamic playback time (to play entire soundclip instead of only for predefined time).
* Allow soundclips to play over each other.
* Provide more feedback to user in terminal window.
