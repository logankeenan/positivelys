use routines_macros::route;
use routines::models::app_request::AppRequest;
use routines::factories::app_response_factory;
use serde_json::Error;
use routines::models::app_response::{AppResponse};
use crate::models::positively::Positively;
use rusqlite::Connection;
use crate::repositories::positivelys_repository::{create_positively, all_positivelys, remove_positively};
use chrono::{Local, Utc};
use rand::seq::SliceRandom;
use std::borrow::Borrow;


#[derive(Deserialize, Serialize)]
pub struct IndexViewModel {
    positivelys: Vec<Positively>,
    todays_total: usize,
    animation_class: String,
}

#[route(path = "/positivelys")]
pub async fn index(app_request: AppRequest) -> IndexViewModel {
    let connection = Connection::open(app_request.app_context.unwrap().database_path).unwrap();
    let positivelys = all_positivelys(&connection);

    let todays_date = Utc::now().with_timezone(&Local).date();
    let todays_total = positivelys.iter().filter(|positively| {
        positively.created_at.with_timezone(&Local).date().eq(&todays_date)
    }).count();

    let animation_classes = vec![
        "bounce-in-left",
        "rotate-in-center",
        "bounce-in-bck",
        "roll-in-left",
        "swirl-in-bottom-bck",
        "slide-in-elliptic-right-fwd",
        "rotate-in-diag-1",
        "jello-horizontal"
    ];

    let option = animation_classes.choose(&mut rand::thread_rng()).unwrap().to_string();
    IndexViewModel {
        positivelys,
        todays_total,
        animation_class: option,
    }
}

#[derive(Deserialize, Serialize)]
pub struct NewViewModel {
    form: Positively
}

#[route(path = "/positivelys/new")]
pub async fn new(_app_request: AppRequest) -> NewViewModel {
    println!("here");
    NewViewModel {
        form: Positively::new()
    }
}

#[route(path = "/positivelys", method = "POST")]
pub async fn create(app_request: AppRequest) -> AppResponse {
    let result: Result<Positively, Error> = serde_json::from_str(app_request.clone().body.unwrap().as_str());
    let connection = Connection::open(app_request.app_context.unwrap().database_path).unwrap();

    match result {
        Ok(positvely) => {
            create_positively(positvely, &connection);
            app_response_factory::redirect("/positivelys".to_string())
        }
        Err(err) => {
            println!("Error: {}", err);

            AppResponse {
                status_code: 200,
                body: Some("failure".to_string()),
                headers: None,
                factory_meta: None,
            }
        }
    }
}

#[route(path = "/positivelys/{id}/delete", method = "POST")]
pub async fn delete(app_request: AppRequest) -> AppResponse {
    let connection = Connection::open(app_request.app_context.clone().unwrap().database_path).unwrap();
    let positively_id = app_request.get_path_param("id").unwrap().parse::<i64>().unwrap();

    remove_positively(&connection, positively_id);

    app_response_factory::redirect("/positivelys".to_string())
}

