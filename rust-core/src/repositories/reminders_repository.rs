use crate::models::reminder::{Reminder, ReminderDay};
use chrono::{NaiveDateTime};
use crate::schema::reminders::columns as reminders_columns;
use crate::schema::reminders::table as reminders_table;
use diesel::{Queryable, Insertable, SqliteConnection, QueryDsl, RunQueryDsl, ExpressionMethods, Identifiable};
use crate::schema::*;
use crate::factories::date_time::{date_time_from_naive, date_time_from_naive_option};
use crate::factories::naive_date_time::current_naive_date_time;

#[derive(Queryable, Identifiable)]
#[table_name = "reminders"]
pub struct ReminderDAO {
    pub id: i32,
    pub minute: i32,
    pub hour: i32,
    pub day: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl ReminderDAO {
    pub fn to_reminder(&self) -> Reminder {
        Reminder {
            id: self.id,
            minute: self.minute,
            hour: self.hour,
            day: self.day,
            day_e: ReminderDay::Everyday,
            created_at: date_time_from_naive(self.created_at),
            updated_at: date_time_from_naive_option(self.updated_at),
        }
    }
}

#[derive(Insertable)]
#[table_name = "reminders"]
pub struct ReminderInsertableDAO {
    pub minute: i32,
    pub hour: i32,
    pub day: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl ReminderInsertableDAO {
    pub fn from_reminder(reminder: Reminder) -> ReminderInsertableDAO {
        ReminderInsertableDAO {
            minute: reminder.minute,
            hour: reminder.hour,
            day: reminder.day,
            created_at: current_naive_date_time(),
            updated_at: None
        }
    }
}

pub fn create(reminder: Reminder, connection: &SqliteConnection) -> Reminder {
    let insertable_dao = ReminderInsertableDAO::from_reminder(reminder);

    diesel::insert_into(reminders_table)
        .values(&insertable_dao)
        .execute(connection).unwrap();

    let recently_created_reminder: ReminderDAO = reminders_table
        .order_by(reminders_columns::id.desc())
        .first(connection).unwrap();

    recently_created_reminder.to_reminder()
}

pub fn all(connection: &SqliteConnection) -> Vec<Reminder> {
    let results: Vec<ReminderDAO> = reminders_table
        .load::<ReminderDAO>(connection)
        .unwrap();

    results.iter().fold(Vec::new(), |mut collection: Vec<Reminder>, reminder_dao| {
        collection.push(reminder_dao.to_reminder());

        collection
    })
}

pub fn remove(connection: &SqliteConnection, id: i32) {
    let _result = diesel::delete(reminders_table.find(id)).execute(connection).unwrap();
}