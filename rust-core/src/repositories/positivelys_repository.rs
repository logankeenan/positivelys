use crate::models::positively::Positively;
use chrono::{DateTime, Utc, NaiveDateTime, Timelike};
use crate::schema::positivelys::columns as positivelys_columns;
use diesel::{Queryable, Insertable, SqliteConnection, QueryDsl, RunQueryDsl, ExpressionMethods};
use crate::schema::*;
use chrono::format::Fixed::TimezoneOffset;

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
    use crate::schema::positivelys::dsl::*;

    let now = chrono::Utc::now();
    let positively_insertable_dao = PositivelyInsertableDAO {
        moment: positively.moment,
        created_at: NaiveDateTime::from_timestamp(now.timestamp(), now.nanosecond()),
        updated_at: None,
    };

    let result = diesel::insert_into(positivelys)
        .values(&positively_insertable_dao)
        .execute(connection)
        .expect("Error saving new post");
}

pub fn all_positivelys_v2(connection: &SqliteConnection) -> Vec<Positively> {
    use crate::schema::positivelys::dsl::*;

    let x: Vec<PositivelyDAO> = positivelys
        .order_by(positivelys_columns::created_at.desc())
        .load::<PositivelyDAO>(connection)
        .unwrap();


    let vec1 = x.iter().fold(Vec::new(), |mut collection: Vec<Positively>, positivelyDoa| {
        let time1: DateTime<Utc> = DateTime::from_utc(positivelyDoa.created_at, Utc);
        let string = time1.to_rfc3339();
        let positively = Positively {
            id: positivelyDoa.id as i64,
            moment: positivelyDoa.moment.to_string(),
            created_at: time1,
            updated_at: None,
        };


        collection.push(positively);

        collection
    });

    vec1

    // vec![]
}

// pub fn remove_positively_v2(connection: &SqliteConnection, id: i64) {
//     let positively_to_delete = positivelys.filter(
//         positivelys_columns::id.eq(id));
//     diesel::delete(positively_to_delete)
//         .execute(&connection.unwrap()).unwrap();
// }
//
// pub fn positively_by_id(connection: &Connection, id: i64) -> Option<Positively> {
//
//
// }

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

// pub fn remove_positively(connection: &Connection, id: i64) {
//     let mut result = connection.prepare("delete from positivelys where id = ?1").unwrap();
//
//     let _rows = result.execute(params![id]);
// }
//
// pub fn update_positively(connection: &Connection, positively: Positively) {
//     let mut result = connection.prepare("update positivelys set moment = ?1, updated_at = ?2 where id = ?3").unwrap();
//
//
//     let _rows = result.execute(params![
//         positively.moment,
//         chrono::Utc::now().timestamp().to_string(),
//         positively.id,
//     ]);
// }
//
// pub fn positively_by_id(connection: &Connection, id: i64) -> Option<Positively> {
//     let mut result = connection.prepare("select id, moment, created_at, updated_at from positivelys where id = ?1").unwrap();
//
//     let rows = result.query_map(params![id], |row| {
//         Ok(Positively {
//             id: row.get(0)?,
//             moment: row.get(1)?,
//             created_at: convert_int_to_datetime(row.get(2)).unwrap(),
//             updated_at: convert_int_to_datetime(row.get(3)),
//         })
//     }).unwrap();
//
//     let positively_result = rows.last().unwrap();
//
//     match positively_result {
//         Ok(positively) => {
//             Some(positively)
//         }
//         Err(_) => {
//             None
//         }
//     }
// }

// fn convert_int_to_datetime(result: Result<i64, Error>) -> Option<DateTime<Utc>> {
//     let updated_at = match result {
//         Ok(date_in_seconds_from_epoch) => {
//             let time = NaiveDateTime::from_timestamp(date_in_seconds_from_epoch, 0);
//             Some(DateTime::<Utc>::from_utc(time, Utc))
//         }
//         Err(_) => {
//             None
//         }
//     };
//     updated_at
// }


// fn convert_int_to_datetime_v2(result: Option<i64>) -> Option<DateTime<Utc>> {
//     let updated_at = match result {
//         Ok(date_in_seconds_from_epoch) => {
//             let time = NaiveDateTime::from_timestamp(date_in_seconds_from_epoch, 0);
//             Some(DateTime::<Utc>::from_utc(time, Utc))
//         }
//         Err(_) => {
//             Option(None)
//         }src/repositories/positivelys_repository.rs:3:20


//     };
//     updated_at
// }