-- Your SQL goes here

CREATE TABLE IF NOT EXISTS positivelys
(
    id         INTEGER PRIMARY KEY not null,
    moment     TEXT NOT NULL,
    created_at int  NOT NULL,
    updated_at int
)