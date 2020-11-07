use handlebars::{Handlebars};
use chrono::{DateTime, Local};

pub fn register_helpers(handlebars: &mut Handlebars) {
    handlebars_helper!(fmt_positively_time: | date_string: str| {
        let result = DateTime::parse_from_rfc3339(date_string);

        match result {
            Ok(date) => {

                let local_time = date.with_timezone(&Local);
                local_time.format("%b %u, %l:%M %p").to_string()
            },
            Err(_) => "".to_string(),
        }
    });
    handlebars.register_helper("fmt_positively_time", Box::new(fmt_positively_time));
}