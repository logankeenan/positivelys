use crate::models::media_file::MediaFile;
use std::fs;
use crate::models::default_properties::current_date_time;
use uuid::Uuid;
use diesel::SqliteConnection;
use crate::repositories::media_files_repository::insert_media_file;

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