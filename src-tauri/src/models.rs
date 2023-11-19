use crate::schema::alarms;
use crate::schema::reminders;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = alarms)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Alarm {
    pub id: i32,
    pub name: String,
    pub hour: String,
    pub days: String,
}

#[derive(Insertable)]
#[diesel(table_name = alarms)]
pub struct NewAlarm<'a> {
    pub name: &'a str,
    pub hour: &'a str,
    pub days: &'a str,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = reminders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Reminder {
    pub id: i32,
    pub name: String,
    pub hour: String,
    pub date: String,
}

#[derive(Insertable)]
#[diesel(table_name = reminders)]
pub struct NewReminder<'a> {
    pub name: &'a str,
    pub hour: &'a str,
    pub date: &'a str,
}
