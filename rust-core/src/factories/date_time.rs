
use chrono::{DateTime, Utc, NaiveDateTime};

pub fn date_time_from_naive(time: NaiveDateTime) -> DateTime<Utc> {
    DateTime::from_utc(time, Utc)
}

pub fn date_time_from_naive_option(time: Option<NaiveDateTime>) -> Option<DateTime<Utc>> {
    match time {
        None => {
            None
        }
        Some(naive_date_time) => {
            Some(date_time_from_naive(naive_date_time))
        }
    }
}