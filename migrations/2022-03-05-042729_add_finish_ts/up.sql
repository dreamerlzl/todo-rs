-- Your SQL goes here
-- ALTER TABLE tasks ADD COLUMN finish_timestamp INTEGER;
CREATE TABLE IF NOT EXISTS histories (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  what TEXT NOT NULL DEFAULT '',
  link VARCHAR(2083),
  finish_timestamp INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS finish_ts_descend ON histories(finish_timestamp);
