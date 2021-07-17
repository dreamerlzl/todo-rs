use core::fmt;
use std::{convert::TryInto, fmt::Display};
use log::{debug};

use rusqlite::{params, Connection}; 
use anyhow::{ Result, Context};

type TodoResult<T> = Result<T>;

pub trait TaskDB {
    fn add_task(&mut self, what: String, link: Option<String>) -> TodoResult<()>;
    fn add_subtask(&mut self, id: u32, what: String, link: Option<String>) -> TodoResult<()>;
    fn get_task(&self, id: u32) -> TodoResult<Option<Task>>;
    fn get_tasks(&self, pattern: Option<String>) -> TodoResult<Vec<Task>>;
    fn get_subtasks(&self, id: u32) -> TodoResult<Vec<SubTask>>;
    fn remove_task(&mut self, id: u32) -> TodoResult<()>;
    fn remove_subtask(&mut self, id: u32, subtask_rank: u32) -> TodoResult<()>;
}

pub struct Task{
    id: u32,
    what: String,
    link: Option<String>,
}

pub fn prompt_task() {
    println!(
        "{0: <10} | {1: <10} | {2: <10}",
        "task_id",
        "description",
        "link(optional)"
    );
}

impl Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.link {
            Some(l) => 
                write!(f, "{0: <10} | {1: <10} | {2: <10}", self.id, self.what, l),
            None => 
                write!(f, "{0: <10} | {1: <10}", self.id, self.what),
        }
    }
}

pub fn prompt_subtask(id: u32) {
    println!("subtask of {}", &id);
    println!(
        "{0: <10} | {1: <10} | {2: <10}",
        "subtask_id",
        "description",
        "link(optional)"
    );
}

pub struct SubTask{
    id: u32,
    what: String,
    link: Option<String>,
    subtask_of: u32,
    subtask_rank: u32,
}

impl Display for SubTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.link {
            Some(l) => 
                write!(f, "{0: <10} | {1: <10} | {2: <10}", self.id, self.what, l),
            None => 
                write!(f, "{0: <10} | {1: <10}", self.id, self.what),
        }
    }
}

pub struct TaskSqlite {
    conn: Connection,
}

impl TaskDB for TaskSqlite {
    fn add_task(&mut self, what: String, link: Option<String>) -> TodoResult<()> {
        self.conn.execute(
            "INSERT INTO task (what, link) VALUES (?1, ?2)",
            params![what, link],
        )?;
        Ok(())
    }

    fn add_subtask(&mut self, id: u32, what: String, link: Option<String>) -> TodoResult<()>{
        let mut stmt = self.conn.prepare("SELECT COUNT(id) FROM subtask WHERE subtask.subtask_of == ?")?;
        let subtask_rank = stmt.query_row(params![id], |row| {
            return row.get(0) as rusqlite::Result<i32>;
        }).unwrap();
        self.conn.execute(
            "INSERT INTO subtask (what, link, subtask_of, subtask_rank) VALUES (?1, ?2, ?3, ?4)", 
            params![what, link, id, subtask_rank],
        )?;
        Ok(())
    }

    fn get_task(&self, id: u32) -> TodoResult<Option<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.what, t.link FROM task t WHERE t.id == ?;").unwrap();
        let mut rows = stmt.query(params![id]).unwrap();
        if let Some(row) = rows.next()? {
            Ok(Some(Task{
                id,
                what: row.get(0).unwrap(),
                link: row.get(1).unwrap(),
            }))
        } else {
            Ok(None)
        }
    }

    fn get_subtasks(&self, id: u32) -> TodoResult<Vec<SubTask>> {
        let mut stmt = self.conn.prepare(
            "select * from subtask s where s.subtask_of == ? order by s.subtask_rank").unwrap();
        let mut rows = stmt.query(params![id]).unwrap();
        let mut subtasks = Vec::new();
        while let Some(row) = rows.next()? {
            subtasks.push(SubTask{
                id: row.get(0)?,
                what: row.get(1)?,
                link: row.get(2)?,
                subtask_of: id,
                subtask_rank: row.get(3)?,    
            });
        }
        Ok(subtasks)
    }

    fn get_tasks(&self, pattern: Option<String>) -> TodoResult<Vec<Task>> {
        let mut stmt =  self.conn.prepare("select * from task").unwrap();
        let rows = stmt.query_map(params![], |row| {
            let task = Task{
                id: row.get(0).unwrap(),
                what: row.get(1).unwrap(),
                link: row.get(2)
                .expect("no link found in the task table row!"),
            };
            debug!("{}", &task);
            Ok(task)
        }).unwrap();
        let rows = if let Some(p) = pattern {
            rows.into_iter()
                .filter_map(|r| 
                    if let Ok(t) = r {if t.what.contains(&p) {Some(t)} else {None}} else {None})
                .collect()
        } else {
            rows.into_iter()
                .filter_map(|r| if let Ok(t) = r { Some(t)} else {None})
                .collect()
        };
        Ok(rows)
    }

    fn remove_task(&mut self, id: u32) -> TodoResult<()> {
        self.conn
            .execute("DELETE FROM task WHERE task.id == ?1", params![&id])
            .expect(&format!("failed to delete {}!", &id));
        self.try_reset_id("task")?;
        Ok(())
    }

    fn remove_subtask(&mut self, id: u32, subtask_rank: u32) -> TodoResult<()> {
        self.conn
            .execute("DELETE FROM subtask WHERE subtask.subtask_of == ?1 AND subtask.subtask_rank == ?2", params![&id, &subtask_rank])
            .expect(&format!("failed to delete the {}-th subtask of {}", &subtask_rank, &id));
        self.try_reset_id("subtask")?;
        Ok(())
    }
}

impl TaskSqlite {
    fn try_reset_id(&mut self, table_name: &str) ->TodoResult<()> {
        let mut stmt = if table_name == "task" {
                self.conn.prepare(
                "SELECT COUNT(id) FROM task")
                .context("fail to count remaining tasks")?
            } else {
                self.conn.prepare(
                    "select count(id) from subtask")
                    .context("fail to count remaining subtasks")?
            }; 
        let task_count = stmt.query_row(params![], |row| {
            return row.get(0) as rusqlite::Result<i32>;
        }).unwrap();
        // try to reset the autoincrement id
        self.conn.execute("UPDATE `sqlite_sequence` SET `seq` = ?1 WHERE `name` = ?2;", 
        params![&task_count, &table_name])
        .context("fail to reset autoincrement id")?;
        Ok(())
    }
}

pub fn open(path: &str) -> TodoResult<Box<dyn TaskDB>> {
    let conn = Connection::open(&path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                what  TEXT NOT NULL DEFAULT '',
                link  VARCHAR(2083)
                )", 
        [],
    ).context("Failed to create table task")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS subtask (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                what TEXT NOT NULL DEFAULT '',
                link VARCHAR(2083),
                subtask_rank int,
                subtask_of int,
                FOREIGN KEY (subtask_of) REFERENCES task(id)
        )",
        [],
    ).context("Failed to create table subtask")?;
    Ok(Box::new(TaskSqlite{ conn }))
}
