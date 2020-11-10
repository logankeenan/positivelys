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

#[macro_use]
extern crate handlebars;

mod controllers;
mod repositories;
mod models;
mod views;

use routines::App;
use routines::models::app_request::{AppRequest, AppContext};
use serde_json::Error;
use futures::executor::block_on;
use crate::repositories::database::{create_database, run_migrations};

async fn handle_request(app_request_json: String, app_context_json: String) -> String {
    let mut app = App::new();
    app.add_route(controllers::positivelys_controller::index);
    app.add_route(controllers::positivelys_controller::create);
    app.add_route(controllers::positivelys_controller::new);
    app.start();

    let app_request_result: Result<AppRequest, Error> = serde_json::from_str(&app_request_json);
    let app_context_result: Result<AppContext, Error> = serde_json::from_str(&app_context_json);

    let app_context = app_context_result.unwrap();
    let connection = create_database(app_context.database_path.clone());
    run_migrations(&connection);

    match app_request_result {
        Ok(mut app_request) => {
            app_request.app_context = Some(app_context);
            let response= app.handle_route(app_request).await;
            json!(response).to_string()
        }
        Err(_) => {
            "500".to_string()
        }
    }
}

pub fn make_request(app_request_json: String, app_context_json: String) -> String {
    block_on(handle_request(app_request_json, app_context_json))
}