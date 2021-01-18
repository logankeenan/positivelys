use chrono::{Utc, DateTime};

#[derive(Debug, Deserialize, Serialize)]
pub struct MediaFile {
    #[serde(default = "crate::models::default_properties::i32_zero")]
    pub id: i32,

    #[serde(default = "crate::models::default_properties::i32_zero")]
    pub positively_id: i32,
    pub file_name: String,
    pub file_location: String,
    pub file_extension: String,

    #[serde(skip_deserializing)]
    #[serde(default = "crate::models::default_properties::current_date_time")]
    pub created_at: DateTime<Utc>,

    pub updated_at: Option<DateTime<Utc>>,
}
