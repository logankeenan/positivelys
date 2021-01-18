use crate::models::media_file::MediaFile;
use std::fs;
use crate::models::default_properties::current_date_time;
use uuid::Uuid;
use diesel::SqliteConnection;
use crate::repositories::media_files_repository::{insert_media_file, media_file_by_positively, delete_media_file_by_positively_id};

pub fn create_media_file(mut temp_file_path: String, media_directory_path: String, positively_id: i32, connection: &SqliteConnection) -> MediaFile {
    let file_name_without_extension = Uuid::new_v4();

    let file_extension = temp_file_path.split(".").collect::<Vec<&str>>()[1].to_string();
    let file_name = format!("{}.{}", file_name_without_extension, file_extension);
    let file_location_saved = format!("{}/{}", media_directory_path, file_name);

    if temp_file_path.starts_with("file://") {
        temp_file_path = temp_file_path.replace("file://", "");
    }

    fs::copy(temp_file_path, file_location_saved.to_string());

    let file_location = format!("/{}", file_name.to_string());

    let file = MediaFile {
        id: 0,
        positively_id,
        file_name,
        file_location,
        file_extension,
        created_at: current_date_time(),
        updated_at: None
    };
    insert_media_file(file, connection)
}

pub fn remove_media_file_file(positively_id: i32, connection: &SqliteConnection, local_files_path: String) {
    let media_file_option = media_file_by_positively(positively_id, &connection);

    match media_file_option {
        None => {}
        Some(media_file) => {
            let remove_file_location = format!("{}{}", local_files_path, media_file.file_location);
            fs::remove_file(remove_file_location);
        }
    }
}

pub fn remove_media_file(positively_id: i32, connection: &SqliteConnection, local_files_path: String) {
    remove_media_file_file(positively_id, &connection, local_files_path);
    delete_media_file_by_positively_id(positively_id, &connection);
}