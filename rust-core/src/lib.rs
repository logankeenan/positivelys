#[macro_use]
extern crate serde_derive;

// TODO - I need to think of a way to not have to do this
// This is required because the macro wants it when I'm generating the
// routes
#[macro_use]
extern crate serde_json;

// TODO - I need to think of a way to not have to do this
// This is required because the macro to generate routes use the include_dir! macro
#[macro_use]
extern crate include_dir;

mod controllers;

use routines::App;
use routines::models::app_request::AppRequest;
use serde_json::Error;
use futures::executor::block_on;

async fn handle_request(app_request_json: String) -> String {
    let mut app = App::new();
    app.add_route(controllers::positivelys_controller::index);
    app.start();

    let app_request_result: Result<AppRequest, Error> = serde_json::from_str(&app_request_json);

    match app_request_result {
        Ok(app_request) => {
            let response= app.handle_route(app_request).await;
            json!(response).to_string()
        }
        Err(_) => {
            "500".to_string()
        }
    }
}

pub fn make_request(app_request_json: String) -> String {
    block_on(handle_request(app_request_json))
}