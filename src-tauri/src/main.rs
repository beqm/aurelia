// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

mod db;
mod evaluate_prompt;
mod models;
mod read_json;
mod schema;

use chrono::Datelike;
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
        .arg("eastus")
        .arg("b483fa3890894357a7f7af78c7522577")
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

fn calculate_date_difference(start_date: &str, end_date: &str) -> Option<String> {
    use chrono::{Datelike, Duration, NaiveDate, Weekday};
    // Parse the input dates
    let start_date = NaiveDate::parse_from_str(start_date, "%d/%m/%Y").ok()?;
    let end_date = NaiveDate::parse_from_str(end_date, "%d/%m/%Y").ok()?;

    // Calculate the difference in days
    let date_difference = end_date.signed_duration_since(start_date).num_days();

    if date_difference > 7 {
        // If the difference is greater than 7 days, return None
        None
    } else if date_difference == 1 {
        // If the difference is 1 day, return "tomorrow"
        Some("amanha".to_string())
    } else {
        // If within 7 days, return the specific day of the week
        let weekday_name = end_date.format("%A").to_string();
        let weekday_name_pt = translate_weekday_to_portuguese(&weekday_name);
        println!("{} {}", weekday_name, weekday_name_pt);
        Some(weekday_name_pt)
    }
}

fn translate_weekday_to_portuguese(weekday_name: &str) -> String {
    match weekday_name {
        "Sun" => "Domingo",
        "Sunday" => "Domingo",
        "Mon" => "Segunda",
        "Monday" => "Segunda",
        "Tue" => "Terca",
        "Tuesday" => "Terca",
        "Wed" => "Quarta",
        "Wednesday" => "Quarta",
        "Thur" => "Quinta",
        "Thursday" => "Quinta",
        "Fri" => "Sexta",
        "Friday" => "Sexta",
        "Sat" => "Sabado",
        "Saturday" => "Sabado",
        _ => weekday_name, // Se não encontrar uma tradução, retorne o original
    }
    .to_string()
}

use chrono::{Local, Timelike, Weekday};
fn get_current_time_and_day() -> (u32, u32, String) {
    // Get the current local time
    let current_time = Local::now().time();

    // Get the current day of the week
    let mut current_day = Local::now().date_naive().weekday();

    let translated_day = translate_weekday_to_portuguese(&current_day.to_string());

    // Extract the hour and minute components
    let current_hour = current_time.hour();
    let current_minute = current_time.minute();

    (current_hour, current_minute, translated_day)
}

#[tauri::command]
fn check_alarm() -> Result<PromptOutput, String> {
    let all_alarms = db::get_alarms();
    let (current_hour, current_minute, current_day) = get_current_time_and_day();

    let mut some_alarm = false;
    for alarm in all_alarms.unwrap() {
        // Parse alarm time
        let alarm_time_parts: Vec<&str> = alarm.hour.split(':').collect();

        if alarm_time_parts.len() == 2 {
            let alarm_hour: u32 = alarm_time_parts[0].parse().unwrap_or_default();
            let alarm_minute: u32 = alarm_time_parts[1].parse().unwrap_or_default();

            // Check if current time matches alarm time and day
            if current_hour == alarm_hour
                && current_minute == alarm_minute
                && alarm.days.contains(&current_day.to_string())
            {
                some_alarm = true
            }
        }
    }
    if some_alarm {
        let status = Command::new("node")
        .arg("../aurelia/synthesis/synthethize.js")
        .arg("../data/output.wav")
        .arg(format!("Sao {}:{}, Pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi, pi", current_hour, current_minute))
        .arg("reg")
        .arg("key")
        .status()
        .map_err(|e| e.to_string())
        .expect("error on synthethize");
        Ok(PromptOutput {
            status: "202".to_string(),
            path: "../data/output.wav".to_string(),
            data: None,
        })
    } else {
        Ok(PromptOutput {
            status: "404".to_string(),
            path: "".to_string(),
            data: None,
        })
    }
}

#[tauri::command]
fn check_reminder() -> Result<PromptOutput, String> {
    use chrono::Local;
    use db::get_reminders;
    let reminders = get_reminders();
    let today = Local::now();
    let formatted_date = today.format("%d/%m/%Y");

    let mut string_msg = String::from("Voce tem um lembrete para");
    let mut can_send = false;

    for r in reminders.unwrap() {
        let result = calculate_date_difference(&formatted_date.to_string(), &r.date);
        println!("{:?}", result.clone());
        if result.is_some() {
            string_msg.push_str(format!(" {},", result.unwrap()).as_str());
            can_send = true;
        }
    }

    if can_send {
        let status = Command::new("node")
            .arg("../aurelia/synthesis/synthethize.js")
            .arg("../data/output.wav")
            .arg(string_msg)
            .arg("reg")
            .arg("key")
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
    } else {
        Ok(PromptOutput {
            status: "404".to_string(),
            path: "".to_string(),
            data: None,
        })
    }
}

#[tauri::command]
fn prompt_response(prompt: &str, supervisioning: bool) -> Result<PromptOutput, String> {
    let input = evaluate(prompt, Some(supervisioning));
    let mut msg = "Desculpe nao entendi, poderia repetir por favor?".to_string();
    let mut state = "";

    if let Ok(eo) = input {
        msg = eo.msg.to_string();
        state = eo.state;
    }

    println!("[INFO] Synthethyzing aurelia speech: {}", msg);

    let status = Command::new("node")
        .arg("../aurelia/synthesis/synthethize.js")
        .arg("../data/output.wav")
        .arg(msg)
        .arg("reg")
        .arg("key")
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        if state == "supervisiontrue" {
            Ok(PromptOutput {
                status: "supervisiontrue".to_string(),
                path: "".to_string(),
                data: None,
            })
        } else if state == "supervisionoff" {
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
        .invoke_handler(tauri::generate_handler![
            prompt_response,
            supervision,
            check_reminder,
            check_alarm
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
