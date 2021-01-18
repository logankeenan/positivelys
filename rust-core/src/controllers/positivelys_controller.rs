use routines_macros::route;
use routines::models::app_request::AppRequest;
use routines::factories::app_response_factory;
use serde_json::Error;
use routines::models::app_response::{AppResponse};
use crate::models::positively::Positively;
use chrono::{Local, Utc};
use rand::seq::SliceRandom;
use crate::repositories::positivelys_repository::{all_positivelys, create_positively, positively_by_id, update_positively, remove_positively};
use crate::repositories::database::establish_connection;
use crate::services::media_files_service::{create_media_file, remove_media_file_file, remove_media_file};
use crate::repositories::media_files_repository::media_file_by_positively;
use crate::models::media_file::MediaFile;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Serialize)]
pub struct IndexViewModel {
    positivelys: Vec<Positively>,
    todays_total: usize,
    animation_class: String,
    assets_path: String,
    local_files_path: String,
}

#[route(path = "/positivelys")]
pub async fn index(app_request: AppRequest) -> IndexViewModel {
    let assets_path = app_request.app_context.clone().unwrap().assets_path;
    let local_files_path = app_request.app_context.clone().unwrap().local_files_path;
    let connection = establish_connection(app_request.app_context.unwrap().database_path);
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
        assets_path,
        local_files_path,
    }
}

#[derive(Deserialize, Serialize)]
pub struct PositivelyForm {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: i32,
    pub moment: String,
    pub media_file_location: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewViewModel {
    form: PositivelyForm,
    assets_path: String,
}

#[route(path = "/positivelys/new")]
pub async fn new(app_request: AppRequest) -> NewViewModel {
    let assets_path = app_request.app_context.clone().unwrap().assets_path;

    NewViewModel {
        form: PositivelyForm {
            id: 0,
            moment: "".to_string(),
            media_file_location: "".to_string(),
        },
        assets_path,
    }
}

#[route(path = "/positivelys", method = "POST")]
pub async fn create(app_request: AppRequest) -> AppResponse {
    let result: Result<PositivelyForm, Error> = serde_json::from_str(app_request.clone().body.unwrap().as_str());
    let local_files_path = app_request.app_context.clone().unwrap().local_files_path;
    let connection = establish_connection(app_request.app_context.unwrap().database_path);
    match result {
        Ok(positively_form) => {
            let mut positively = Positively::new();
            positively.moment = positively_form.moment;

            let positively_saved = create_positively(positively, &connection);

            if !positively_form.media_file_location.is_empty() {
                create_media_file(
                    positively_form.media_file_location,
                    local_files_path,
                    positively_saved.id,
                    &connection,
                );
            }

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
    let local_files_path = app_request.app_context.clone().unwrap().local_files_path;

    remove_media_file_file(positively_id, &connection, local_files_path);
    remove_positively(&connection, positively_id);

    app_response_factory::redirect("https://positivelys.com/positivelys".to_string())
}

#[derive(Deserialize, Serialize)]
pub struct EditViewModel {
    form: PositivelyForm,
    assets_path: String,
    local_files_path: String,
}

#[route(path = "/positivelys/{id}/edit", method = "GET")]
pub async fn edit(app_request: AppRequest) -> EditViewModel {
    let assets_path = app_request.app_context.clone().unwrap().assets_path;
    let connection = establish_connection(app_request.app_context.clone().unwrap().database_path);
    let positively_id = app_request.get_path_param("id").unwrap().parse::<i32>().unwrap();
    let local_files_path = app_request.app_context.clone().unwrap().local_files_path;
    let positively = positively_by_id(&connection, positively_id).unwrap();
    let media_file_option = media_file_by_positively(positively_id, &connection);

    println!("local_files_path: {}", local_files_path);

    EditViewModel {
        form: PositivelyForm {
            id: positively.id,
            moment: positively.moment,
            media_file_location: match media_file_option {
                None => "".to_string(),
                Some(media_file) => media_file.file_location
            },
        },
        assets_path,
        local_files_path,
    }
}

#[route(path = "/positivelys/{id}/edit", method = "POST")]
pub async fn update(app_request: AppRequest) -> AppResponse {
    let local_files_path = app_request.app_context.clone().unwrap().local_files_path;
    let connection = establish_connection(app_request.app_context.clone().unwrap().database_path);
    let mut positively_form: PositivelyForm = serde_json::from_str(app_request.clone().body.unwrap().as_str()).unwrap();
    let mut positively = positively_by_id(&connection, positively_form.id).unwrap();
    let positively_id = positively.id;
    positively.moment = positively_form.moment;

    update_positively(&connection, positively);

    if positively_form.media_file_location.is_empty() {
        remove_media_file(positively_id, &connection, local_files_path);
    } else {
        let existing_media_file = media_file_by_positively(positively_id, &connection);

        match existing_media_file {
            None => {
                create_media_file(
                    positively_form.media_file_location,
                    local_files_path,
                    positively_id,
                    &connection,
                );
            }
            Some(media_file) => {
                let media_is_different = !positively_form.media_file_location.ends_with(media_file.file_location.as_str());

                if media_is_different {
                    remove_media_file(positively_id, &connection, local_files_path.to_string());
                    create_media_file(
                        positively_form.media_file_location,
                        local_files_path,
                        positively_id,
                        &connection,
                    );
                }
            }
        }
    }


    app_response_factory::redirect("https://positivelys.com/positivelys".to_string())
}

