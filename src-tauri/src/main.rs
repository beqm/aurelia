// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

mod db;
mod evaluate_prompt;
mod models;
mod read_json;
mod schema;

use evaluate_prompt::evaluate;

use std::process::Command;

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
struct PromptOutput {
    status: String,
    path: String,
    data: Option<Vec<String>>,
}

use screenshots::Screen;

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
            data: None,
        })
    } else {
        Err("Failed to run synthesize.ts".to_string())
    }
}

fn check_keywords(strings: Vec<String>) -> (bool, Option<String>) {
    // Hardcoded vector of variations for keywords related to gaming, watching videos, series, movies, and animes
    let forbidden_keywords = vec![
        "game",
        "gaming",
        "games",
        "gamer",
        "player",
        "playing",
        "play",
        "stream",
        "streaming",
        "streamer",
        "watching",
        "watch",
        "videos",
        "video",
        "series",
        "movies",
        "animes",
        "anime",
        "twitch",
        "kick",
        "youtube",
        "twitter",
    ];
    // Convert the vector of keywords to a HashSet for efficient lookup
    let keyword_set: std::collections::HashSet<_> = forbidden_keywords.iter().cloned().collect();

    // Check if any forbidden keyword is present in the vector of strings
    for s in &strings {
        if keyword_set.contains(s.as_str()) {
            println!("found value: {}", s.as_str());
            return (
                true,
                Some(format!(
                    "Voce nao deveria estar fazendo isso, volte a focar!",
                )),
            );
        }
    }

    // If no forbidden keyword is found
    (false, None)
}

use crate::read_json::read_json_file;
#[tauri::command]
fn supervision() -> Result<PromptOutput, String> {
    let _ = screenshot_display();
    let keywords = read_json_file("../data/result.json");
    let msg = check_keywords(keywords.unwrap());

    let status = Command::new("node")
        .arg("../aurelia/synthesis/synthethize.js")
        .arg("../data/output.wav")
        .arg(msg.1.unwrap())
        .arg("region")
        .arg("key")
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        if msg.0 == false {
            Ok(PromptOutput {
                status: "focus".to_string(),
                path: "../data/output.wav".to_string(),
                data: None,
            })
        } else {
            Ok(PromptOutput {
                status: "not-focus".to_string(),
                path: "../data/output.wav".to_string(),
                data: None,
            })
        }
    } else {
        Err("Failed to run synthesize.ts".to_string())
    }
}

#[tauri::command]
fn prompt_response(prompt: &str, supervisioning: bool) -> Result<PromptOutput, String> {
    let input = evaluate(prompt, Some(supervisioning)).unwrap();

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
        if input.state == "supervisiontrue" {
            Ok(PromptOutput {
                status: "supervisiontrue".to_string(),
                path: "".to_string(),
                data: None,
            })
        } else if input.state == "supervisionoff" {
            Ok(PromptOutput {
                status: "supervisionoff".to_string(),
                path: "".to_string(),
                data: None,
            })
        } else {
            Ok(PromptOutput {
                status: "202".to_string(),
                path: "../data/output.wav".to_string(),
                data: None,
            })
        }
    } else {
        Err("Failed to run synthesize.ts".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![prompt_response, supervision,])
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
