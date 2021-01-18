use routines_macros::route;
use routines::models::app_request::AppRequest;
use routines::factories::app_response_factory;
use serde_json::Error;
use routines::models::app_response::{AppResponse};
use crate::models::positively::Positively;
// use crate::repositories::positivelys_repository::{create_positively, all_positivelys, remove_positively, positively_by_id, update_positively};
use chrono::{Local, Utc};
use rand::seq::SliceRandom;
use std::ops::Deref;
use crate::repositories::positivelys_repository::{all_positivelys_v2, create_positively_v2, positively_by_id, update_positively, remove_positively};
use crate::repositories::database::establish_connection;

#[derive(Deserialize, Serialize)]
pub struct IndexViewModel {
    positivelys: Vec<Positively>,
    todays_total: usize,
    animation_class: String,
}

#[route(path = "/positivelys")]
pub async fn index(app_request: AppRequest) -> IndexViewModel {
    let connection = establish_connection(app_request.app_context.unwrap().database_path);
    let positivelys = all_positivelys_v2(&connection);

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
    NewViewModel {
        form: Positively::new()
    }
}

#[route(path = "/positivelys", method = "POST")]
pub async fn create(app_request: AppRequest) -> AppResponse {
    let result: Result<Positively, Error> = serde_json::from_str(app_request.clone().body.unwrap().as_str());
    let connection = establish_connection(app_request.app_context.unwrap().database_path);

    match result {
        Ok(positvely) => {
            create_positively_v2(positvely, &connection);


            // let positivelys = all_positivelys(&connection);
            // let option = positivelys.get(0).unwrap().to_owned();
            //
            // let url = format!("https://positivelys.com/positivelys/{}/edit", option.id);
            // app_response_factory::redirect(url.to_string())

            app_response_factory::redirect("https://positivelys.com/positivelys".to_string())
        }
        Err(_) => {
            AppResponse {
                status_code: 200,
                body: Some("failure".to_string()),
                headers: None,
                factory_meta: None,
            }
        }
    }
}
//
#[route(path = "/positivelys/{id}/delete", method = "POST")]
pub async fn delete(app_request: AppRequest) -> AppResponse {
    let connection = establish_connection(app_request.app_context.clone().unwrap().database_path);
    let positively_id = app_request.get_path_param("id").unwrap().parse::<i32>().unwrap();

    remove_positively(&connection, positively_id);

    app_response_factory::redirect("https://positivelys.com/positivelys".to_string())
}
//
#[derive(Deserialize, Serialize)]
pub struct EditViewModel {
    form: Positively
}
//
#[route(path = "/positivelys/{id}/edit", method = "GET")]
pub async fn edit(app_request: AppRequest) -> EditViewModel {
    let connection = establish_connection(app_request.app_context.clone().unwrap().database_path);
    let positively_id = app_request.get_path_param("id").unwrap().parse::<i32>().unwrap();

    let positively = positively_by_id(&connection, positively_id).unwrap();

    EditViewModel {
        form: positively
    }
}
//
#[route(path = "/positivelys/{id}/edit", method = "POST")]
pub async fn update(app_request: AppRequest) -> AppResponse {
let connection = establish_connection(app_request.app_context.clone().unwrap().database_path);
    let positively_id = app_request.get_path_param("id").unwrap().parse::<i32>().unwrap();
    let mut positively: Positively = serde_json::from_str(app_request.clone().body.unwrap().as_str()).unwrap();

    positively.id = positively_id;
    update_positively(&connection, positively);

    app_response_factory::redirect("https://positivelys.com/positivelys".to_string())
}

