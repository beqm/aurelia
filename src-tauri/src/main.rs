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
        .invoke_handler(tauri::generate_handler![prompt_response])
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
