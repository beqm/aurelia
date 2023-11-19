// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rodio::{source::Source, Decoder, OutputStream};
use std::env;
use std::error::Error;
use std::fs::File;
use std::i16;
use std::io::BufReader;

mod evaluate_prompt;
use evaluate_prompt::evaluate;

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

use std::process::Command;

#[tauri::command]
fn prompt_response(prompt: &str) -> Result<(), String> {
    let input = evaluate(prompt).unwrap();

    println!(
        "[INFO] Synthethyzing aurelia speech: {}",
        input.msg.as_str()
    );
    let status = Command::new("node")
        .arg("../aurelia/synthesis/synthethize.js")
        .arg("../output.wav")
        .arg(input.msg.as_str())
        // Botar key e region do azure
        .arg("region")
        .arg("key")
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to run synthesize.ts".to_string())
    }
}

// #[tauri::command]
// fn save_audio(jsData: Vec<u8>) -> &'static str {
//     let array = js_sys::Uint8Array::new(&jsData);
//     let bytes: Vec<u8> = array.to_vec();
//     match save_wav("../output.wav", &bytes, 44100) {
//         Ok(()) => {
//             println!("WAV file created successfully.");
//             "ok"
//         }
//         Err(err) => {
//             eprintln!("Error: {}", err);
//             "error"
//         }
//     }
// }

// fn save_wav(filename: &str, data: &[u8], sample_rate: u32) -> Result<(), Box<dyn Error>> {
//     let mut writer = hound::WavWriter::create(
//         filename,
//         hound::WavSpec {
//             channels: 1,
//             sample_rate: sample_rate,
//             bits_per_sample: 16, // Change to 16 bits per sample
//             sample_format: hound::SampleFormat::Int,
//         },
//     )?;

//     for &sample in data {
//         // Convert u8 to i16, clamping the value to the valid range for i16
//         let sample_i16 = i16::from(sample).clamp(i16::MIN, i16::MAX);
//         writer.write_sample(sample_i16)?;
//     }

//     writer.finalize()?;
//     Ok(())
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![startup_voice, prompt_response])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
