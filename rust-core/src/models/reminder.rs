use chrono::{DateTime, Utc};
use crate::models::default_properties::current_date_time;

#[derive(Debug, Deserialize, Serialize)]
#[derive(strum_macros::Display)]
pub enum ReminderDay {
    Everyday = -3,
    Weekends = -2,
    Weekdays = -1,
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reminder {
    #[serde(default = "crate::models::default_properties::i32_zero")]
    pub id: i32,
    pub minute: i32,
    pub hour: i32,
    pub day: i32,
    pub day_e: ReminderDay,

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
            day: 0,
            day_e: ReminderDay::Everyday,
            created_at: current_date_time(),
            updated_at: None,
        }
    }
}


