use crate::models::positively::Positively;
use rusqlite::{params, Connection, Error};
use chrono::{DateTime, Utc, NaiveDateTime};

pub fn create_positively_table(connection: &Connection) {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS positivelys (
                  id              INTEGER PRIMARY KEY,
                  moment            TEXT NOT NULL,
                  created_at int NOT NULL,
                  updated_at int
                  )",
        params![],
    ).unwrap();
}

//TODO is this the right way to store dates in sqlite? Shouldn't I have it by the millisecond?
pub fn create_positively(positively: Positively, connection: &Connection) {
    connection.execute(
        "insert into positivelys (moment, created_at) values (?1, ?2)",
        params![positively.moment, positively.created_at.timestamp().to_string()],
    ).unwrap();
}


pub fn all_positivelys(connection: &Connection) -> Vec<Positively> {
    let mut result = connection.prepare("select id, moment, created_at, updated_at from positivelys order by created_at desc").unwrap();

    let rows = result.query_map(params![], |row| {
        Ok(Positively {
            id: row.get(0)?,
            moment: row.get(1)?,
            created_at: convert_int_to_datetime(row.get(2)).unwrap(),
            updated_at: convert_int_to_datetime(row.get(3)),
        })
    }).unwrap();

    let map: Vec<Positively> = rows.map(|p| { p.unwrap() }).collect();

    map
}

pub fn remove_positively(connection: &Connection, id: i64) {
    let mut result = connection.prepare("delete from positivelys where id = ?1").unwrap();

    let rows = result.execute(params![id]);

    println!("{}", rows.unwrap());
}

fn convert_int_to_datetime(result: Result<i64, Error>) -> Option<DateTime<Utc>> {
    let updated_at = match result {
        Ok(date_in_seconds_from_epoch) => {
            let time = NaiveDateTime::from_timestamp(date_in_seconds_from_epoch, 0);
            Some(DateTime::<Utc>::from_utc(time, Utc))
        }
        Err(_) => {
            None
        }
    };
    updated_at
}