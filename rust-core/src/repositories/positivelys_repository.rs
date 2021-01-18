use crate::models::positively::Positively;
use chrono::{DateTime, Utc, NaiveDateTime, Timelike};
use crate::schema::positivelys::columns as positivelys_columns;
use crate::schema::positivelys::table as positivelys_table;
use diesel::{Queryable, Insertable, SqliteConnection, QueryDsl, RunQueryDsl, ExpressionMethods, QueryResult};
use crate::schema::*;
use chrono::format::Fixed::TimezoneOffset;
use diesel::result::Error;

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

pub fn create_positively_v2(positively: Positively, connection: &SqliteConnection) {
    let now = chrono::Utc::now();
    let positively_insertable_dao = PositivelyInsertableDAO {
        moment: positively.moment,
        created_at: NaiveDateTime::from_timestamp(now.timestamp(), now.nanosecond()),
        updated_at: None,
    };

    let result = diesel::insert_into(positivelys_table)
        .values(&positively_insertable_dao)
        .execute(connection)
        .expect("Error saving new post");
}

pub fn all_positivelys_v2(connection: &SqliteConnection) -> Vec<Positively> {
    let x: Vec<PositivelyDAO> = positivelys_table
        .order_by(positivelys_columns::created_at.desc())
        .load::<PositivelyDAO>(connection)
        .unwrap();


    let vec1 = x.iter().fold(Vec::new(), |mut collection: Vec<Positively>, positivelyDoa| {
        let positively = Positively {
            id: positivelyDoa.id,
            moment: positivelyDoa.moment.to_string(),
            created_at: DateTimeFromNaive(positivelyDoa.created_at),
            updated_at: None,
        };


        collection.push(positively);

        collection
    });

    vec1
}

fn DateTimeFromNaive(time: NaiveDateTime) -> DateTime<Utc> {
    DateTime::from_utc(time, Utc)
}

// pub fn remove_positively_v2(connection: &SqliteConnection, id: i64) {
//     let positively_to_delete = positivelys.filter(
//         positivelys_columns::id.eq(id));
//     diesel::delete(positively_to_delete)
//         .execute(&connection.unwrap()).unwrap();
// }
//
pub fn positively_by_id(connection: &SqliteConnection, id: i32) -> Option<Positively> {
    let x: QueryResult<PositivelyDAO> = positivelys_table.find(id).first(connection);

    match x {
        Ok(positivelys_doa) => {
            let positively = Positively {
                id: positivelys_doa.id,
                moment: positivelys_doa.moment,
                created_at: DateTimeFromNaive(positivelys_doa.created_at),
                updated_at: None,
            };

            Some(positively)
        }
        Err(_) => {
            None
        }
    }
}

// pub fn create_positively_table(connection: &Connection) {
//     connection.execute(
//         "CREATE TABLE IF NOT EXISTS positivelys (
//                   id              INTEGER PRIMARY KEY,
//                   moment            TEXT NOT NULL,
//                   created_at int NOT NULL,
//                   updated_at int
//                   )",
//         params![],
//     ).unwrap();
// }

// //TODO is this the right way to store dates in sqlite? Shouldn't I have it by the millisecond?
// pub fn create_positively(positively: Positively, connection: &Connection) {
//     connection.execute(
//         "insert into positivelys (moment, created_at) values (?1, ?2)",
//         params![positively.moment, positively.created_at.timestamp().to_string()],
//     ).unwrap();
// }
//
//
// pub fn all_positivelys(connection: &Connection) -> Vec<Positively> {
//     let mut result = connection.prepare("select id, moment, created_at, updated_at from positivelys order by created_at desc").unwrap();
//
//     let rows = result.query_map(params![], |row| {
//         Ok(Positively {
//             id: row.get(0)?,
//             moment: row.get(1)?,
//             created_at: convert_int_to_datetime(row.get(2)).unwrap(),
//             updated_at: convert_int_to_datetime(row.get(3)),
//         })
//     }).unwrap();
//
//     let map: Vec<Positively> = rows.map(|p| { p.unwrap() }).collect();
//
//     map
// }

pub fn remove_positively(connection: &SqliteConnection, id: i32) {
    let result = diesel::delete(positivelys_table.find(id)).execute(connection);

    match result {
        Ok(success) => {
            println!("update positively success: {}", success)
        }
        Err(error) => {
            println!("update positively error: {}", error)
        }
    }
}

pub fn update_positively(connection: &SqliteConnection, positively: Positively) {
    let now = chrono::Utc::now();
    let time = NaiveDateTime::from_timestamp(now.timestamp(), now.nanosecond());
    let result = diesel::update(positivelys_table.find(positively.id)).set((
        positivelys_columns::moment.eq(positively.moment),
        positivelys_columns::updated_at.eq(Some(time))
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
