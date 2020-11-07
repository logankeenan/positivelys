use routines_macros::route;
use routines::models::app_request::AppRequest;
use routines::factories::app_response_factory;
use serde_json::Error;
use routines::models::app_response::{AppResponse};
use crate::models::positively::Positively;
use rusqlite::Connection;
use crate::repositories::positivelys_repository::{create_positively, all_positivelys};


#[derive(Deserialize, Serialize)]
pub struct IndexViewModel {
    positivelys: Vec<Positively>
}

#[route(path = "/positivelys")]
pub async fn index(app_request: AppRequest) -> IndexViewModel {
    let connection = Connection::open(app_request.app_context.unwrap().database_path).unwrap();
    let positivelys = all_positivelys(&connection);

    println!("length: {}", positivelys.len());
    IndexViewModel {
        positivelys
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
                factory_meta: None
            }
        }
    }
}

