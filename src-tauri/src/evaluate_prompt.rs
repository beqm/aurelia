use crate::db::{self, delete_alarm};
use crate::models::Alarm;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
enum ActionType {
    Set,
    Remove,
    None,
}

#[derive(Debug)]
pub enum EventData {
    Alarm { hour: String, days: Vec<DayOfWeek> },
    Reminder { hour: String, date: String },
}

#[derive(Debug, Serialize, Deserialize)]
enum DayOfWeek {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

#[derive(Debug)]
pub struct EventOperation {
    pub typ: &'static str,
    pub msg: String,
    pub state: &'static str,
    pub data: Option<EventData>,
}

fn parse_days_string(days_str: &str) -> Vec<&str> {
    serde_json::from_str(days_str).unwrap_or_default()
}

fn vec_to_alarm_string(alarms: &Vec<Alarm>) -> String {
    alarms
        .iter()
        .map(|alarm| {
            let days_vec = parse_days_string(&alarm.days);
            format!("alarme {} para {}", alarm.hour, days_vec.join(", "))
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn evaluate(input: &str) -> Result<EventOperation, &'static str> {
    let normalized_input = input.to_lowercase();

    let is_list_event = normalized_input.contains("liste") || normalized_input.contains("mostre");

    let is_alarm_event = normalized_input.contains("alarme") | normalized_input.contains("alarmes");
    let is_reminder_event =
        normalized_input.contains("lembrete") || normalized_input.contains("data");

    if is_list_event && is_alarm_event {
        let all_alarms = db::get_alarms().unwrap();
        let stringified_alarms = vec_to_alarm_string(&all_alarms);

        if all_alarms.len() != 0 {
            return Ok(EventOperation {
                typ: "Desconhecido",
                msg: format!(
                    "Ok, vou falar todos os seus alarmes, {}, esses sao seus alarmes",
                    stringified_alarms
                ),
                state: "Get",
                data: None,
            });
        } else {
            return Ok(EventOperation {
                typ: "Desconhecido",
                msg: format!("Atualmente voce nao tem nenhum alarme registrado",),
                state: "Get",
                data: None,
            });
        }
    } else if is_list_event && is_reminder_event {
        let all_reminders = db::get_reminders().unwrap();

        if all_reminders.len() != 0 {
            let strinfigied_reminders = all_reminders
                .iter()
                .map(|reminder| format!("lembrete para {}", reminder.name))
                .collect::<Vec<String>>()
                .join("\n");

            return Ok(EventOperation {
                typ: "Desconhecido",
                msg: format!(
                    "Ok, vou falar todos os seus lembretes, {}, esses sao seus lembretes",
                    strinfigied_reminders
                ),
                state: "Get",
                data: None,
            });
        } else {
            return Ok(EventOperation {
                typ: "Desconhecido",
                msg: format!("Atualmente voce nao tem nenhum lembrete registrado",),
                state: "Get",
                data: None,
            });
        }
    }

    let time_re = Regex::new(r"(\d{1,2}:\d{2}\s*[ap]m)").unwrap();
    let time_match = time_re.find(&normalized_input);

    let date_re = Regex::new(r"(\d{1,2}/\d{1,2}/\d{4})").unwrap();
    let date_match = date_re.find(&normalized_input);

    let mut action_type: ActionType = ActionType::None;

    if normalized_input.contains("remover") || normalized_input.contains("delete") {
        action_type = ActionType::Remove
    }

    if normalized_input.contains("setar")
        || normalized_input.contains("sete")
        || normalized_input.contains("faca")
        || normalized_input.contains("crie")
    {
        action_type = ActionType::Set
    }

    let event_data = if is_alarm_event && time_match.is_some() {
        let hour = time_match.unwrap().as_str().to_string();
        // Check if it mentions "todos os dias"
        let days = if input.to_lowercase().contains("todos os dias") {
            vec![
                DayOfWeek::Domingo,
                DayOfWeek::Segunda,
                DayOfWeek::Terca,
                DayOfWeek::Quarta,
                DayOfWeek::Quinta,
                DayOfWeek::Sexta,
                DayOfWeek::Sabado,
            ]
        } else {
            // Extract specified days of the week
            let days_re =
                Regex::new(r"(domingo|segunda|terça|quarta|quinta|sexta|sabado)").unwrap();
            let days: Vec<DayOfWeek> = days_re
                .find_iter(input)
                .filter_map(|m| match m.as_str() {
                    "domingo" => Some(DayOfWeek::Domingo),
                    "segunda" => Some(DayOfWeek::Segunda),
                    "terça" => Some(DayOfWeek::Terca),
                    "quarta" => Some(DayOfWeek::Quarta),
                    "quinta" => Some(DayOfWeek::Quinta),
                    "sexta" => Some(DayOfWeek::Sexta),
                    "sabado" => Some(DayOfWeek::Sabado),
                    _ => None,
                })
                .collect();

            days
        };
        EventData::Alarm { hour, days }
    } else if is_reminder_event && time_match.is_some() && date_match.is_some() {
        let hour = time_match.unwrap().as_str().to_string();
        let date = date_match.unwrap().as_str().to_string();
        EventData::Reminder { hour, date }
    } else {
        return Ok(EventOperation {
            typ: "Desconhecido",
            msg: "Desculpe, não entendi.".to_string(),
            state: "Desconhecido",
            data: None,
        });
    };

    let event_operation: EventOperation = match (event_data, action_type) {
        (EventData::Alarm { hour, days }, ActionType::Set) => {
            // If it's a set action for an alarm
            let msg = if days.is_empty() {
                format!("Configurando um alarme para {} todos os dias", hour)
            } else {
                let days_str: Vec<String> = days.iter().map(|d| format!("{:?}", d)).collect();
                format!(
                    "Configurando um alarme para {} nos dias, {}",
                    hour,
                    days_str.join(", ")
                )
            };
            db::create_alarm(&hour, &hour, &serde_json::to_string(&days).unwrap());
            EventOperation {
                typ: "Alarme",
                msg,
                state: "Criar",
                data: Some(EventData::Alarm { hour, days }),
            }
        }
        (EventData::Reminder { hour, date }, ActionType::Set) => {
            // If it's a set action for a reminder
            let msg = format!("Configurando um lembrete para {} em {}", hour, date);
            db::create_reminder(&format!("{hour}, {date}"), &hour, &date);
            EventOperation {
                typ: "Lembrete",
                msg,
                state: "Criar",
                data: Some(EventData::Reminder { hour, date }),
            }
        }
        (_, ActionType::Remove) if is_alarm_event => {
            // If it's a remove action for an alarm
            let hour = time_match.unwrap().as_str().to_string();

            // Add logic to remove the alarm with the specified hour and days
            let result = db::delete_alarm(&hour);
            match result {
                Some(c) => EventOperation {
                    typ: "Alarme",
                    msg: format!(
                        "Removendo o alarme para {} setado para os dias, {}",
                        c.name, c.days
                    ),
                    state: "Remover",
                    data: None,
                },
                None => EventOperation {
                    typ: "Lembrete",
                    msg: format!("Nao achei nenhum alarme com a descricao que voce me deu"),
                    state: "Remover",
                    data: None,
                },
            }
        }
        (_, ActionType::Remove) if is_reminder_event => {
            // If it's a remove action for a reminder
            let hour = time_match.unwrap().as_str().to_string();
            let date = date_match.unwrap().as_str().to_string();

            // Add logic to remove the reminder with the specified hour and date
            let result = db::delete_reminder(format!("{hour}, {date}"));
            if result.is_some() {
                EventOperation {
                    typ: "Lembrete",
                    msg: format!("Removendo o lembrete para {} em {}", hour, date),
                    state: "Remover",
                    data: Some(EventData::Reminder { hour, date }),
                }
            } else {
                EventOperation {
                    typ: "Lembrete",
                    msg: format!("Nao achei nenhum lembrete com a descricao que voce me deu."),
                    state: "Remover",
                    data: Some(EventData::Reminder { hour, date }),
                }
            }
        }
        (_, ActionType::Remove) => {
            // If it's a remove action for an unknown event type
            return Err("Combinação inválida de palavras-chave e tipo de ação.");
        }
        _ => {
            return Err("Combinação inválida de palavras-chave e tipo de ação.");
        }
    };

    Ok(event_operation)
}
