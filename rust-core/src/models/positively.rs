use chrono::{DateTime, Utc};
use crate::models::default_properties::current_date_time;

#[derive(Deserialize, Serialize)]
pub struct Positively {
    #[serde(default = "crate::models::default_properties::i64_zero")]
    pub id: i64,
    pub moment: String,

    #[serde(skip_deserializing)]
    #[serde(default = "crate::models::default_properties::current_date_time")]
    pub created_at: DateTime<Utc>,

    pub updated_at: Option<DateTime<Utc>>,
}

impl Positively {
    pub fn new() -> Self {
        Self {
            id: 0,
            moment: "".to_string(),
            created_at: current_date_time(),
            updated_at: None
        }
    }
}


