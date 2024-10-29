use notify_rust::Notification;
use rodio::{Decoder, OutputStream, Source};
use std::fs::{self, File};
use std::io::BufReader;
use std::{env, thread};
use std::time::Duration;
use chrono::prelude::*;
use toml::Value;

fn play_audio(file_path: &str) {
    // Get a speaker stream handle to send sounds to
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Load a sound from a file, using the provided file path
    let file = BufReader::new(File::open(file_path).unwrap());

    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // Keep the thread alive while the sound is playing.
    // This is a simple way to wait for the audio to finish playing.
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let duration_minutes = args.get(1)
        .expect("Please provide the duration in minutes as the first argument.")
        .parse::<i64>()
        .expect("Please provide a valid number for the duration.");

    // Read the TOML configuration file
    let config_string = fs::read_to_string("configs/config.toml")
        .expect("Failed to read config.toml");

    // Parse the TOML configuration
    let config: Value = config_string.parse::<Value>()
        .expect("Failed to parse config.toml");

    // Extract the sound file path and clone it to be used in the thread, because the thread may outlive the main function's scope
    let sound_file = config.get("sound")
        .and_then(|s| s.get("file"))
        .and_then(|f| f.as_str())
        .expect("Sound file path is not specified in config.toml")
        .to_owned(); // Clone the string here

    // Schedule the sound to play after the specified duration
    let when = Local::now() + chrono::Duration::minutes(duration_minutes);
    println!("Scheduling sound to play at {}", when);

    let duration_until_play = (when - Local::now()).to_std().unwrap();

    thread::spawn(move || {
        thread::sleep(duration_until_play);
        if let Err(e) = Notification::new()
            .summary("Reminder:")
            .body("Trink was, beweg dich, geh raus")
            .icon("dialog-information")
            .timeout(10000) // milliseconds
            .show() {
                println!("Failed to show notification: {}", e);
            }
        play_audio(&sound_file); // Use the cloned string here
    });

    // Instead of sleeping the main thread, we can wait for the spawned thread to finish
    let handle = thread::spawn(move || {
        thread::sleep(duration_until_play + Duration::from_secs(5));
    });

    handle.join().unwrap();
}
