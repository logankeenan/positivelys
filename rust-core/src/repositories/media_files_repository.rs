use chrono::NaiveDateTime;
use crate::schema::media_files::columns as media_files_columns;
use crate::schema::media_files::table as media_files_table;
use crate::models::media_file::MediaFile;
use diesel::{SqliteConnection, RunQueryDsl, Connection, QueryDsl, ExpressionMethods, Associations, Identifiable};
use crate::repositories::positivelys_repository::{current_naive_date_time, date_time_from_naive, date_time_from_naive_option};
use crate::schema::*;
use diesel::result::Error;

#[derive(Debug, Queryable, Associations)]
#[belongs_to(crate::repositories::positivelys_repository::PositivelyDAO, foreign_key = "positively_id")]
#[table_name = "media_files"]
pub struct MediaFileDAO {
    pub id: i32,
    pub positively_id: i32,
    pub file_name: String,
    pub file_location: String,
    pub file_extension: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "media_files"]
pub struct MediaFileInsertableDAO {
    pub positively_id: i32,
    pub file_name: String,
    pub file_location: String,
    pub file_extension: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl MediaFileDAO {
    pub fn to_media_file(&self) -> MediaFile {
        MediaFile {
            id: self.id,
            positively_id: self.positively_id,
            file_name: self.file_name.to_string(),
            file_location: self.file_location.to_string(),
            file_extension: self.file_extension.to_string(),
            created_at: date_time_from_naive(self.created_at.clone()),
            updated_at: date_time_from_naive_option(self.updated_at.clone()),
        }
    }
}

pub fn insert_media_file(media_file: MediaFile, connection: &SqliteConnection) -> MediaFile {
    let result = connection.transaction::<_, Error, _>(|| {
        let media_file_insertable = MediaFileInsertableDAO {
            positively_id: media_file.positively_id,
            file_name: media_file.file_name,
            file_location: media_file.file_location,
            file_extension: media_file.file_extension,
            created_at: current_naive_date_time(),
            updated_at: None,
        };

        diesel::insert_into(media_files_table)
            .values(&media_file_insertable)
            .execute(connection);


        let saved_positively: MediaFileDAO = media_files_table
            .order_by(media_files_columns::id.desc())
            .first(connection).unwrap();

        Ok(MediaFile {
            id: saved_positively.id,
            positively_id: saved_positively.positively_id,
            file_name: saved_positively.file_name,
            file_location: saved_positively.file_location,
            file_extension: saved_positively.file_extension,
            created_at: date_time_from_naive(saved_positively.created_at.clone()),
            updated_at: None,
        })
    });

    result.unwrap()
}