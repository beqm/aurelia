// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rodio::{source::Source, Decoder, OutputStream};
use std::env;
use std::fs::File;
use std::io::BufReader;

#[tauri::command]
fn startup_voice() -> &'static str {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("../aurelia/voice/startup/welcome.wav").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    let _ = stream_handle.play_raw(source.convert_samples());
    return "OK";
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![startup_voice])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
