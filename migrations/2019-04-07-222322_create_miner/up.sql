-- Your SQL goes here
CREATE TABLE miners
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    host     VARCHAR(100)                      NOT NULL,
    username VARCHAR(100)                      NOT NULL,
    password VARCHAR(100)                      NOT NULL
)