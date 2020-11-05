use chrono::{NaiveDateTime, DateTime, Utc};

#[derive(Deserialize, Serialize)]
pub struct Positively {

    #[serde(default = "crate::models::default_properties::i64_zero")]
    pub id: i64,
    pub moment: String,

    #[serde(skip_deserializing)]
    #[serde(default = "crate::models::default_properties::current_date_time")]
    pub created_at: DateTime<Utc>,

    pub updated_at: Option<DateTime<Utc>>
}
