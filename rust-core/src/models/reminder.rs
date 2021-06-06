use chrono::{DateTime, Utc};
use crate::models::default_properties::current_date_time;

#[derive(Debug, Deserialize, Serialize)]
#[derive(strum_macros::Display)]
pub enum ReminderDay {
    Everyday = -3,
    Weekdays = -2,
    Weekends = -1,
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

impl ReminderDay {
    pub fn from(value: i32) -> ReminderDay {
        match value {
            -3 => { ReminderDay::Everyday }
            -2 => { ReminderDay::Weekdays }
            -1 => { ReminderDay::Weekends }
            0 => { ReminderDay::Sunday }
            1 => { ReminderDay::Monday }
            2 => { ReminderDay::Tuesday }
            3 => { ReminderDay::Wednesday }
            4 => { ReminderDay::Thursday }
            5 => { ReminderDay::Friday }
            6 => { ReminderDay::Saturday }
            _ => {
                // This should never happen
                ReminderDay::Saturday
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reminder {
    #[serde(default = "crate::models::default_properties::i32_zero")]
    pub id: i32,
    pub minute: i32,
    pub hour: i32,
    pub day: ReminderDay,

    #[serde(skip_deserializing)]
    #[serde(default = "crate::models::default_properties::current_date_time")]
    pub created_at: DateTime<Utc>,

    pub updated_at: Option<DateTime<Utc>>,
}

impl Reminder {
    pub fn new() -> Self {
        Self {
            id: 0,
            minute: 0,
            hour: 0,
            day: ReminderDay::Saturday,
            created_at: current_date_time(),
            updated_at: None,
        }
    }
}


