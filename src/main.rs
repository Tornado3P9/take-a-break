use notify_rust::Notification;
use rodio::{Decoder, OutputStream, Source};
use std::fs::create_dir_all;
use std::fs::{self, File};  // use std::path::Path;
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

    // Define the path to the configuration file
    let config_dir = dirs::home_dir().unwrap().join(".config/take-a-break");
    let config_file_path = config_dir.join("config.toml");

    // Check if the configuration directory exists, if not, create it
    if !config_dir.exists() {
        create_dir_all(&config_dir).expect("Failed to create configuration directory");
    }

    // Check if the configuration file exists, if not, create it with default content
    if !config_file_path.exists() {
        let default_config = r#"[sound]
file = "path/to/sounds/new-message.wav"

[notification]
body = "Time for a break! Stretch, hydrate, or take a short walk."
"#;
        fs::write(&config_file_path, default_config).expect("Failed to create default config.toml");

        println!("The config.toml file has been created at: {}", config_file_path.display());
        println!("Please edit the config.toml file and then re-run the program.");
        return; // Exit the program to allow the user to edit the config file
    }

    // Read the TOML configuration file
    let config_string = fs::read_to_string(config_file_path)
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

    // Extract the notification body and clone it to be used in the thread
    let notification_body = config.get("notification")
        .and_then(|n| n.get("body"))
        .and_then(|b| b.as_str())
        .expect("Notification body is not specified in config.toml")
        .to_owned(); // Clone the string here

    // Schedule the sound to play after the specified duration
    let when = Local::now() + chrono::Duration::minutes(duration_minutes);
    println!("Scheduling sound to play at {}", when);

    let duration_until_play = (when - Local::now()).to_std().unwrap();

    thread::spawn(move || {
        thread::sleep(duration_until_play);
        if let Err(e) = Notification::new()
            .summary("Reminder:")
            .body(&notification_body)
            .icon("dialog-information")
            .timeout(10000) // milliseconds
            .show() {
                println!("Failed to show notification: {}", e);
            }
        play_audio(&sound_file);
    });

    // Instead of sleeping the main thread, we can wait for the spawned thread to finish
    let handle = thread::spawn(move || {
        thread::sleep(duration_until_play + Duration::from_secs(5));
    });

    handle.join().unwrap();
}
