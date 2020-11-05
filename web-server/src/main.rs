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

use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse, HttpRequest, HttpMessage, http};
use rust_core::make_request;
use routines::models::app_request::AppRequest;
use routines::routing::RouteDefinitionMethod;
use routines::models::app_response::AppResponse;
use serde_json::Error;
use actix_web::test::read_body;
use actix_web::web::Bytes;

#[get("{url}")]
async fn index(web::Path(url): web::Path<String>) -> impl Responder {
    let request = AppRequest {
        path: format!("/{}", url),
        method: RouteDefinitionMethod::GET,
        body: None,
        app_context: None,
        headers: None
    };

    let json = json!({
	    "database_path": "./database.sqlite"
    });

    let response_as_json = make_request(json!(request).to_string(), json.to_string());
    let result: Result<AppResponse, Error> = serde_json::from_str(&response_as_json);
    match result {
        Ok(response) => {
            HttpResponse::Ok().body(response.body.unwrap())
        }
        Err(_) => {
            HttpResponse::Ok().body("error")
        }
    }
}

#[post("{url}")]
async fn post_route(web::Path(url): web::Path<String>, bytes: Bytes) -> impl Responder {
    let a = match String::from_utf8(bytes.to_vec()) {
        Ok(text) => text,
        Err(_) => "error".to_string()
    };

    println!("{},", a.clone());

    let request = AppRequest {
        path: format!("/{}", url),
        method: RouteDefinitionMethod::POST,
        body: Some(a),
        app_context: None,
        headers: None
    };

    let json = json!({
	    "database_path": "./database.sqlite"
    });


    println!("post_route before make request");
    let response_as_json = make_request(json!(request).to_string(), json.to_string());
    println!("after post_route before make request");
    let result: Result<AppResponse, Error> = serde_json::from_str(&response_as_json);
    match result {
        Ok(response) => {
            if response.status_code == 302 {
                let option = response.headers.unwrap().get("Location").unwrap().to_string();

                println!("option: {}", option.to_string());
                let result1 = HttpResponse::Found()
                    .header(http::header::LOCATION, option.to_string()).finish();

                return result1
            }

            HttpResponse::Ok().body(response.body.unwrap())
        }
        Err(_) => {

            HttpResponse::Ok().body("error")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Server running at http://localhost:8080");
    HttpServer::new(|| App::new().service(index).service(post_route))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}