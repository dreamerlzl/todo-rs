-- Your SQL goes here
PRAGMA foreign_keys=ON;
CREATE TABLE IF NOT EXISTS subtasks (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  what TEXT NOT NULL DEFAULT '',
  link VARCHAR(2083),
  subtask_rank int NOT NULL,
  task_id int NOT NULL,
  FOREIGN KEY (task_id) REFERENCES tasks(id)
)
