use chrono::{DateTime, Utc};

pub fn i32_zero() -> i32 {
    0
}

pub fn current_date_time() -> DateTime<Utc> {
    chrono::Utc::now()
}