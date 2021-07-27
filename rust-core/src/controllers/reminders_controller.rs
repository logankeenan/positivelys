use routines::models::app_request::AppRequest;
use routines::models::app_response::AppResponse;
use routines_macros::route;
use serde_json::Error;
use routines::factories::html_factory;
use routines::factories::app_response_factory;
use crate::models::reminder::{Reminder, ReminderDay};
use crate::repositories::reminders_repository;
use crate::repositories::database::establish_connection;
use serde_aux::field_attributes::deserialize_number_from_string;
use crate::factories::naive_date_time::from_24_hour;
use chrono::Timelike;

#[derive(Serialize)]
pub struct IndexJSONViewModel {
    reminders: Vec<Reminder>
}

#[route(path = "/reminders", content_type = "application/json")]
pub async fn index_as_json(app_request: AppRequest) -> IndexJSONViewModel {
    let app_context = app_request.clone().app_context.unwrap();
    let connection = establish_connection(app_context.database_path);
    let reminders = reminders_repository::all(&connection);

    IndexJSONViewModel {
        reminders
    }
}


#[derive(Serialize)]
pub struct IndexViewModel {
    form: ReminderForm,
    reminders: Vec<Reminder>,
    assets_path: String,
    local_files_path: String,
}

#[route(path = "/reminders")]
pub async fn index(app_request: AppRequest) -> IndexViewModel {
    let app_context = app_request.clone().app_context.unwrap();
    let local_files_path = app_context.local_files_path;
    let connection = establish_connection(app_context.database_path);
    let reminders = reminders_repository::all(&connection);

    IndexViewModel {
        form: ReminderForm {
            time: "12:00".to_string(),
            day: 1,
        },
        reminders,
        assets_path: html_factory::assets_path(app_request),
        local_files_path,
    }
}

#[derive(Deserialize, Serialize)]
pub struct ReminderForm {
    pub time: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub day: i32,
}

#[route(path = "/reminders", method = "POST")]
pub async fn create(app_request: AppRequest) -> AppResponse {
    let result: Result<ReminderForm, Error> = serde_json::from_str(app_request.clone().body.unwrap().as_str());
    let connection = establish_connection(app_request.app_context.unwrap().database_path);
    match result {
        Ok(reminder_form) => {
            //TODO handle this failure
            let time = from_24_hour(reminder_form.time).unwrap();
            let mut reminder = Reminder::new();
            reminder.hour = time.hour() as i32;
            reminder.minute = time.minute() as i32;
            reminder.day = ReminderDay::from(reminder_form.day);

            let _saved_reminder = reminders_repository::create(reminder, &connection);
            app_response_factory::redirect("https://positivelys.com/reminders".to_string())
        }
        Err(_) => {
            AppResponse {
                status_code: 200,
                body: Some("failure".to_string()),
                headers: None,
            }
        }
    }
}

#[route(path = "/reminders/{id}/delete", method = "POST")]
pub async fn delete(app_request: AppRequest) -> AppResponse {
    let connection = establish_connection(app_request.app_context.clone().unwrap().database_path);
    let reminder_id = app_request.get_path_param("id").unwrap().parse::<i32>().unwrap();
    reminders_repository::remove(&connection, reminder_id);

    app_response_factory::redirect("https://positivelys.com/reminders".to_string())
}

