use crate::models::positively::Positively;
use chrono::{NaiveDateTime};
use crate::schema::positivelys::columns as positivelys_columns;
use crate::schema::positivelys::table as positivelys_table;
use crate::schema::media_files::table as media_files_table;
use crate::schema::media_files::columns as media_files_columns;
use diesel::{Queryable, Insertable, SqliteConnection, QueryDsl, RunQueryDsl, ExpressionMethods, QueryResult, Connection, JoinOnDsl, Identifiable};
use crate::schema::*;
use diesel::result::Error;
use crate::repositories::media_files_repository::MediaFileDAO;
use crate::factories::date_time::{date_time_from_naive, date_time_from_naive_option};
use crate::factories::naive_date_time::current_naive_date_time;

#[derive(Queryable, Identifiable)]
#[table_name = "positivelys"]
pub struct PositivelyDAO {
    pub id: i32,
    pub moment: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "positivelys"]
pub struct PositivelyInsertableDAO {
    pub moment: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

pub fn create_positively(positively: Positively, connection: &SqliteConnection) -> Positively {
    let result = connection.transaction::<_, Error, _>(|| {
        let positively_insertable_dao = PositivelyInsertableDAO {
            moment: positively.moment,
            created_at: current_naive_date_time(),
            updated_at: None,
        };

        diesel::insert_into(positivelys_table)
            .values(&positively_insertable_dao)
            .execute(connection).unwrap();

        let saved_positively: PositivelyDAO = positivelys_table
            .order_by(positivelys_columns::id.desc())
            .first(connection).unwrap();

        Ok(Positively {
            id: saved_positively.id,
            moment: saved_positively.moment.to_string(),
            media_file: None,
            created_at: date_time_from_naive(saved_positively.created_at.clone()),
            updated_at: None,
        })
    });

    result.unwrap()
}

pub fn all_positivelys(connection: &SqliteConnection) -> Vec<Positively> {
    let results: Vec<(PositivelyDAO, Option<MediaFileDAO>)> = positivelys_table
        .order_by(positivelys_columns::created_at.desc())
        .left_outer_join(media_files_table.on(media_files_columns::positively_id.eq(positivelys_columns::id)))
        .load::<(PositivelyDAO, Option<MediaFileDAO>)>(connection)
        .unwrap();

    results.iter().fold(Vec::new(), |mut collection: Vec<Positively>, (positively_doa, media_file_doa)| {
        let positively = Positively {
            id: positively_doa.id,
            moment: positively_doa.moment.to_string(),
            media_file: match media_file_doa {
                None => None,
                Some(media_file) => Some(media_file.to_media_file())
            },
            created_at: date_time_from_naive(positively_doa.created_at),
            updated_at: date_time_from_naive_option(positively_doa.updated_at),
        };

        collection.push(positively);

        collection
    })
    // vec![]
}

pub fn positively_by_id(connection: &SqliteConnection, id: i32) -> Option<Positively> {
    let result: QueryResult<PositivelyDAO> = positivelys_table.find(id).first(connection);

    match result {
        Ok(positivelys_doa) => {
            let positively = Positively {
                id: positivelys_doa.id,
                moment: positivelys_doa.moment,
                media_file: None,
                created_at: date_time_from_naive(positivelys_doa.created_at),
                updated_at: date_time_from_naive_option(positivelys_doa.updated_at),
            };

            Some(positively)
        }
        Err(_) => {
            None
        }
    }
}

pub fn remove_positively(connection: &SqliteConnection, id: i32) {
    let _result = diesel::delete(positivelys_table.find(id)).execute(connection).unwrap();
}

pub fn update_positively(connection: &SqliteConnection, positively: Positively) {
    let result = diesel::update(positivelys_table.find(positively.id)).set((
        positivelys_columns::moment.eq(positively.moment),
        positivelys_columns::updated_at.eq(Some(current_naive_date_time()))
    )).execute(connection);

    match result {
        Ok(success) => {
            println!("update positively success: {}", success)
        }
        Err(error) => {
            println!("update positively error: {}", error)
        }
    }
}
