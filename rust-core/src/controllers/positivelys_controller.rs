use routines_macros::route;
use routines::models::app_request::AppRequest;

#[derive(Deserialize, Serialize)]
pub struct SomeStruct {
    name: String
}

#[route(path = "/positivelys")]
pub async fn index(_app_request: AppRequest) -> SomeStruct {
    SomeStruct {
        name: "Logan Keenan".to_string()
    }
}