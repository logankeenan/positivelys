use chrono::{DateTime, Utc};

pub fn i64_zero() -> i64 {
    0
}

pub fn current_date_time() -> DateTime<Utc> {
    chrono::Utc::now()
}