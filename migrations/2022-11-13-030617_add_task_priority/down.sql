-- This file should undo anything in `up.sql`
PRAGMA foreign_keys=off;

BEGIN TRANSACTION;

ALTER TABLE priority RENAME TO _priority_old;

CREATE TABLE tasks (
  id    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  what  TEXT NOT NULL DEFAULT '',
  link  VARCHAR(2083)
);

INSERT INTO priority (id, what, link)
  SELECT id, what, link
  FROM _priority_old;

COMMIT;

PRAGMA foreign_keys=on;
