// @generated automatically by Diesel CLI.

diesel::table! {
    alarms (id) {
        id -> Integer,
        name -> Text,
        hour -> Text,
        days -> Text,
    }
}

diesel::table! {
    reminders (id) {
        id -> Integer,
        name -> Text,
        hour -> Text,
        date -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    alarms,
    reminders,
);
