use crate::models::positively::Positively;
use chrono::{DateTime, Utc, NaiveDateTime, Timelike};
use crate::schema::positivelys::columns as positivelys_columns;
use crate::schema::positivelys::table as positivelys_table;
use diesel::{Queryable, Insertable, SqliteConnection, QueryDsl, RunQueryDsl, ExpressionMethods, QueryResult};
use crate::schema::*;

#[derive(Queryable)]
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

pub fn create_positively(positively: Positively, connection: &SqliteConnection) {
    let positively_insertable_dao = PositivelyInsertableDAO {
        moment: positively.moment,
        created_at: current_naive_date_time(),
        updated_at: None,
    };

    let _result = diesel::insert_into(positivelys_table)
        .values(&positively_insertable_dao)
        .execute(connection)
        .expect("Error saving new post");
}

pub fn all_positivelys(connection: &SqliteConnection) -> Vec<Positively> {
    let results: Vec<PositivelyDAO> = positivelys_table
        .order_by(positivelys_columns::created_at.desc())
        .load::<PositivelyDAO>(connection)
        .unwrap();


    results.iter().fold(Vec::new(), |mut collection: Vec<Positively>, positively_doa| {
        let positively = Positively {
            id: positively_doa.id,
            moment: positively_doa.moment.to_string(),
            created_at: date_time_from_naive(positively_doa.created_at),
            updated_at: date_time_from_naive_option(positively_doa.updated_at),
        };

        collection.push(positively);

        collection
    })
}

pub fn positively_by_id(connection: &SqliteConnection, id: i32) -> Option<Positively> {
    let result: QueryResult<PositivelyDAO> = positivelys_table.find(id).first(connection);

    match result {
        Ok(positivelys_doa) => {
            let positively = Positively {
                id: positivelys_doa.id,
                moment: positivelys_doa.moment,
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

fn date_time_from_naive(time: NaiveDateTime) -> DateTime<Utc> {
    DateTime::from_utc(time, Utc)
}

fn current_naive_date_time() -> NaiveDateTime {
    let now = chrono::Utc::now();
    let time = NaiveDateTime::from_timestamp(now.timestamp(), now.nanosecond());
    time
}

fn date_time_from_naive_option(time: Option<NaiveDateTime>) -> Option<DateTime<Utc>> {
    match time {
        None => {
            None
        }
        Some(naive_date_time) => {
            Some(date_time_from_naive(naive_date_time))
        }
    }
}
