// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

mod db;
mod evaluate_prompt;
mod models;
mod schema;

use evaluate_prompt::evaluate;
use std::process::Command;

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
struct PromptOutput {
    status: String,
    path: String,
}

use screenshots::Screen;

#[tauri::command]
fn screenshot_display() -> Result<PromptOutput, String> {
    let screens = Screen::all().unwrap();

    for screen in screens {
        println!("capturer {screen:?}");
        let image = screen.capture().unwrap();
        image.save(format!("../data/display.png")).unwrap();
    }

    let status = Command::new("python")
        .arg("../aurelia/vision/analyze-image.py")
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(PromptOutput {
            status: "202".to_string(),
            path: "../data/output.wav".to_string(),
        })
    } else {
        Err("Failed to run synthesize.ts".to_string())
    }
}

#[tauri::command]
fn prompt_response(prompt: &str) -> Result<PromptOutput, String> {
    let input = evaluate(prompt).unwrap();

    println!(
        "[INFO] Synthethyzing aurelia speech: {}",
        input.msg.as_str()
    );

    let status = Command::new("node")
        .arg("../aurelia/synthesis/synthethize.js")
        .arg("../data/output.wav")
        .arg(input.msg.as_str())
        .arg("region")
        .arg("key")
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(PromptOutput {
            status: "202".to_string(),
            path: "../data/output.wav".to_string(),
        })
    } else {
        Err("Failed to run synthesize.ts".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            prompt_response,
            screenshot_display
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| match event {
            tauri::RunEvent::WindowEvent {
                label,
                event: win_event,
                ..
            } => match win_event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    app.exit(0);
                }

                _ => {}
            },
            _ => {}
        })
}
