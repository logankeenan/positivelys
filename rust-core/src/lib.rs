#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

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
mod services;
mod models;
mod views;
mod schema;

use routines::App;
use routines::models::app_request::{AppRequest};
use serde_json::Error;
use futures::executor::block_on;
use crate::repositories::database::{establish_connection, run_migrations};
use std::collections::HashMap;
use routines::models::app_context::AppContext;

async fn handle_request(app_request_json: String, app_context_json: String) -> String {
    let mut app = App::new();
    app.add_route(controllers::positivelys_controller::index);
    app.add_route(controllers::positivelys_controller::create);
    app.add_route(controllers::positivelys_controller::new);
    app.add_route(controllers::positivelys_controller::edit);
    app.add_route(controllers::positivelys_controller::update);
    app.add_route(controllers::positivelys_controller::delete);
    app.start();

    let app_request_result: Result<AppRequest, Error> = serde_json::from_str(&app_request_json);
    let app_context_result: Result<AppContext, Error> = serde_json::from_str(&app_context_json);

    let app_context = app_context_result.unwrap();
    let connection = establish_connection(app_context.database_path.clone());
    run_migrations(&connection);

    match app_request_result {
        Ok(mut app_request) => {
            app_request.app_context = Some(app_context);
            let request_uri = app_request.uri.to_string();
            let mut response = app.handle_route(app_request).await;

            match response.headers {
                None => {
                    let mut headers = HashMap::new();
                    headers.insert("Content-Location".to_string(), request_uri);
                    response.headers = Some(headers);

                }
                Some(ref mut headers) => {
                    headers.insert("Content-Location".to_string(), request_uri);
                }
            }

            json!(response).to_string()
        }
        Err(error) => {
            println!("error: {}", error);
            "500".to_string()
        }
    }
}

pub fn make_request(app_request_json: String, app_context_json: String) -> String {
    block_on(handle_request(app_request_json, app_context_json))
}