use chrono::{NaiveDateTime, Timelike, ParseError};
use chrono::format::Numeric::Timestamp;
use std::time::SystemTime;

pub fn current_naive_date_time() -> NaiveDateTime {
    let now = chrono::Utc::now();
    let time = NaiveDateTime::from_timestamp(now.timestamp(), now.nanosecond());
    time
}

pub fn from_24_hour(time: String) -> Option<NaiveDateTime> {
    let some_arbitrary_date_to_satisfy_contructor = "07/08/01";
    let string = format!("{} {}", some_arbitrary_date_to_satisfy_contructor, time.as_str());

    match NaiveDateTime::parse_from_str(&string, "%D %H:%M") {
        Ok(date) => {
            Some(date)
        }
        Err(error) => {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::factories::naive_date_time::from_24_hour;
    use chrono::Timelike;

    #[test]
    fn it_should_parse_24_hour_clock_to_naive_date_time() {
        let option = from_24_hour("13:30".to_string());
        let time = option.unwrap();

        assert_eq!(time.hour(), 13);
        assert_eq!(time.minute(), 30);
    }

    #[test]
    fn it_should_parse_24_hour_clock_to_with_am_hour_to_naive_date_time() {
        let option = from_24_hour("05:55".to_string());
        let time = option.unwrap();

        assert_eq!(time.hour(), 5);
        assert_eq!(time.minute(), 55);
    }
}