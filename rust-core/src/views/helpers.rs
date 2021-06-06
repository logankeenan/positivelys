use handlebars::{Handlebars};
use chrono::{DateTime, Local, Utc, Timelike, TimeZone};

pub fn register_helpers(handlebars: &mut Handlebars) {
    handlebars_helper!(fmt_positively_time: | date_string: str| {
        let result = DateTime::parse_from_rfc3339(date_string);

        match result {
            Ok(date) => {

                let local_time = date.with_timezone(&Local);
                local_time.format("%b %e, %l:%M %p").to_string()
            },
            Err(_) => "".to_string(),
        }
    });
    handlebars.register_helper("fmt_positively_time", Box::new(fmt_positively_time));

    handlebars_helper!(fmt_reminder_time: | hour: i64, minute: i64 | {
        reminder_time(hour, minute)
    });

    handlebars.register_helper("fmt_reminder_time", Box::new(fmt_reminder_time));
}

fn reminder_time(hour: i64, minute: i64) -> String {
    let time = chrono::Local.ymd(2000, 1, 1).and_hms(hour as u32, minute as u32, 0);

    time.format("%l:%M %p").to_string()
}

#[cfg(test)]
mod tests {
    use crate::views::helpers::reminder_time;

    #[test]
    fn it_should_render_reminder_time() {
        assert_eq!(reminder_time(1, 30), " 1:30 AM");
        assert_eq!(reminder_time(13, 30), " 1:30 PM");
        assert_eq!(reminder_time(0, 0), "12:00 AM");
    }

}