use chrono::{DateTime, Utc};
use crate::models::default_properties::current_date_time;
use crate::models::media_file::MediaFile;

#[derive(Debug, Deserialize, Serialize)]
pub struct Positively {
    #[serde(default = "crate::models::default_properties::i32_zero")]
    pub id: i32,
    pub moment: String,
    pub media_file: Option<MediaFile>,
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
            media_file: None,
            created_at: current_date_time(),
            updated_at: None
        }
    }
}


