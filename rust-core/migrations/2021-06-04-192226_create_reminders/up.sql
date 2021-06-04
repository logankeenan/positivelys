-- Your SQL goes here

create table if not exists reminders
(
    id    INTEGER PRIMARY KEY not null,
    minute INTEGER not null,
    hour INTEGER not null,
    day INTEGER not null,
    created_at INTEGER not null,
    updated_at INTEGER
)