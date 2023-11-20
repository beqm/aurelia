use crate::models::Alarm;
use crate::models::NewAlarm;
use crate::models::NewReminder;
use crate::models::Reminder;
use diesel::prelude::*;
use diesel::SqliteConnection;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations/");

pub fn establish_connection() -> SqliteConnection {
    // println!("[INFO] Connecting to database");

    // Build url
    // let database_url = "_up_/data/data.db";

    // Dev url
    let database_url = "../data/data.db";
    let mut conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    conn.run_pending_migrations(MIGRATIONS).unwrap();
    conn
}

pub fn create_alarm(name: &str, hour: &str, days: &str) -> Option<Alarm> {
    use crate::schema::alarms;
    let mut conn = establish_connection();

    let new_alarm = NewAlarm { name, hour, days };

    let result = diesel::insert_or_ignore_into(alarms::table)
        .values(&new_alarm)
        .returning(Alarm::as_returning())
        .get_result(&mut conn)
        .optional();

    match result {
        Ok(c) => {
            println!("[SUCCESS] Creating new alarm: {:?}", &name);
            return c;
        }
        Err(err) => {
            println!("[ERROR] Failed Creating new alarm: {:?}", err);
            return None;
        }
    }
}

pub fn get_alarms() -> Option<Vec<Alarm>> {
    use crate::schema::alarms::dsl::*;
    let mut conn = establish_connection();
    let result = alarms.select(Alarm::as_select()).get_results(&mut conn);

    match result {
        Ok(c) => {
            // println!("[SUCCESS] Fetching all alarms");
            return Some(c);
        }
        Err(_) => {
            println!("[ERROR] Alarms not found");
            return None;
        }
    }
}

pub fn delete_alarm(alarm_name: &str) -> Option<Alarm> {
    use crate::schema::alarms::dsl::*;
    let mut conn = establish_connection();
    let result = diesel::delete(alarms.filter(name.eq(&alarm_name))).get_result(&mut conn);

    match result {
        Ok(c) => {
            println!("[SUCCESS] Deleting alarm: {:?}", &alarm_name);
            return Some(c);
        }
        Err(_) => {
            println!("[ERROR] Alarm not found: {:?}", &alarm_name);
            return None;
        }
    }
}

pub fn create_reminder(name: &str, hour: &str, date: &str) -> Option<Reminder> {
    use crate::schema::reminders;
    let mut conn = establish_connection();

    let new_reminder = NewReminder { name, hour, date };

    let result = diesel::insert_or_ignore_into(reminders::table)
        .values(&new_reminder)
        .returning(Reminder::as_returning())
        .get_result(&mut conn)
        .optional();

    match result {
        Ok(c) => {
            println!("[SUCCESS] Creating new reminder: {:?}", &name);
            return c;
        }
        Err(err) => {
            println!("[ERROR] Failed creating new reminder: {:?}", err);
            return None;
        }
    }
}

pub fn get_reminders() -> Option<Vec<Reminder>> {
    use crate::schema::reminders::dsl::*;
    let mut conn = establish_connection();
    let result = reminders
        .select(Reminder::as_select())
        .get_results(&mut conn);

    match result {
        Ok(c) => {
            println!("[SUCCESS] Fetching all reminders");
            return Some(c);
        }
        Err(_) => {
            println!("[ERROR] Reminders not found");
            return None;
        }
    }
}

pub fn delete_reminder(reminder_name: String) -> Option<Reminder> {
    use crate::schema::reminders::dsl::*;
    let mut conn = establish_connection();
    let result = diesel::delete(reminders.filter(name.eq(&reminder_name))).get_result(&mut conn);

    match result {
        Ok(c) => {
            println!("[SUCCESS] Deleting reminder: {:?}", &reminder_name);
            return Some(c);
        }
        Err(_) => {
            println!("[ERROR] Reminder not found: {:?}", &reminder_name);
            return None;
        }
    }
}
