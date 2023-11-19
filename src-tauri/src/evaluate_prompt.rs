use regex::Regex;

#[derive(Debug)]
enum ActionType {
    Set,
    Remove,
}

#[derive(Debug)]
enum EventData {
    Alarm { time: String },
    Reminder { date: String, time: String },
}

#[derive(Debug)]
pub struct EventOperation {
    typ: &'static str,
    pub msg: String,
    state: &'static str,
}

pub fn evaluate(input: &str) -> Result<EventOperation, &'static str> {
    // Check if the input contains the keywords "alarm" or "reminder"
    let contains_alarm = input.to_lowercase().contains("alarm");
    let contains_reminder = input.to_lowercase().contains("reminder");

    // Use regex to extract time expressions like "8:00 am"
    let time_re = Regex::new(r"(\d{1,2}:\d{2}\s*[ap]m)").unwrap();
    let time_match = time_re.find(input);

    // Use regex to extract date expressions like "24th of January 2025"
    let date_re = Regex::new(r"(\d{1,2}(st|nd|rd|th) of \w+ \d{4})").unwrap();
    let date_match = date_re.find(input);

    // Check if the input contains keywords for setting or removing an event
    let action_type = if input.to_lowercase().contains("remove") {
        ActionType::Remove
    } else {
        ActionType::Set
    };

    let event_data = if contains_alarm && time_match.is_some() {
        // If the input contains "alarm" and a time expression
        EventData::Alarm {
            time: time_match.unwrap().as_str().to_string(),
        }
    } else if contains_reminder && time_match.is_some() && date_match.is_some() {
        // If the input contains "reminder," a time expression, and a date expression
        let date = date_match.unwrap().as_str().to_string();
        let time = time_match.unwrap().as_str().to_string();
        EventData::Reminder { date, time }
    } else {
        // If none of the expected keywords are found
        return Ok(EventOperation {
            typ: "Unknown",
            msg: "Desculpe, nao consegui entender, tem como voce repetir?".to_string(),
            state: "Unknown",
        });
    };

    let event_operation = match (event_data, action_type) {
        (EventData::Alarm { time }, ActionType::Set) => {
            // If it's a set action for an alarm
            let msg = format!("Setando alarme para {}", time);
            EventOperation {
                typ: "Alarm",
                msg,
                state: "Create",
            }
        }
        (EventData::Reminder { date, time }, ActionType::Set) => {
            // If it's a set action for a reminder
            let msg = format!("Setando uma data para {} at {}", date, time);
            EventOperation {
                typ: "Reminder",
                msg,
                state: "Create",
            }
        }
        (_, ActionType::Remove) => {
            // If it's a remove action for either alarm or reminder
            let msg = "Deletando o evento".to_string();
            EventOperation {
                typ: "Event",
                msg,
                state: "Remove",
            }
        }
        _ => return Err("Invalid combination of keywords and action type."),
    };

    Ok(event_operation)
}
