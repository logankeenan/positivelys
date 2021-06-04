-- Your SQL goes here

CREATE TABLE IF NOT EXISTS media_files
(
    id         INTEGER PRIMARY KEY not null,
    positively_id integer not null,
    file_name     TEXT NOT NULL,
    file_location TEXT not null,
    file_extension TEXT not null,
    created_at int  NOT NULL,
    updated_at int,
    FOREIGN KEY(positively_id) REFERENCES positivelys(id) on delete cascade
)