-- Your SQL goes here

CREATE TABLE IF NOT EXISTS media_files
(
    id         INTEGER PRIMARY KEY,
    positively_id integer,
    file_name     TEXT NOT NULL,
    file_location TEXT not null,
    file_extension TEXT not null,
    created_at int  NOT NULL,
    updated_at int,
    FOREIGN KEY(positively_id) REFERENCES positivelys(id) on delete cascade
)