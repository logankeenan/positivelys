use chrono::{DateTime, Utc};
use crate::models::default_properties::current_date_time;

#[derive(Debug, Deserialize, Serialize)]
pub struct Reminder {
    #[serde(default = "crate::models::default_properties::i32_zero")]
    pub id: i32,
    pub minute: i32,
    pub hour: i32,
    pub day: i32,

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
            created_at: current_date_time(),
            updated_at: None
        }
    }
}


