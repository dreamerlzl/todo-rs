use super::schema::{histories, subtasks, tasks};
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use diesel::Queryable;

use std::fmt::Display;

#[derive(Identifiable, Debug, Queryable, PartialEq, Eq)]
#[table_name = "tasks"]
pub struct Task {
    pub id: i32,
    pub what: String,
    pub link: Option<String>,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
    pub what: String,
    pub link: Option<String>,
}

#[derive(Debug, PartialEq, Identifiable, Queryable, Associations)]
#[belongs_to(Task)]
#[table_name = "subtasks"]
pub struct SubTask {
    pub id: i32,
    pub what: String,
    pub link: Option<String>,
    #[warn(dead_code)]
    task_id: i32,
    subtask_rank: i32,
}

#[derive(Insertable)]
#[table_name = "subtasks"]
pub struct NewSubTask {
    pub what: String,
    pub link: Option<String>,
    pub task_id: i32,
    pub subtask_rank: i32,
}

#[derive(Debug, PartialEq, Identifiable, Queryable)]
#[table_name = "histories"]
pub struct History {
    pub id: i32,
    pub what: String,
    pub link: Option<String>,
    pub finish_timestamp: i32,
}

#[derive(Insertable)]
#[table_name = "histories"]
pub struct NewHistory {
    pub what: String,
    pub link: Option<String>,
    pub finish_timestamp: i32,
}

macro_rules! my_format {
    (task) => {
        "{0: <10} {1: <50} {2: <10}"
    };
    (subtask) => {
        "{0: <10} {1: <50} {2: <10}"
    };
    (subtask) => {};
    (history) => {
        "{: <15} {: <50} {}"
    };
    (id_history) => {
        "{: <10} {: <15} {: <50} {}"
    };
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.link {
            Some(l) => write!(f, my_format!(task), self.id, self.what, l),
            None => write!(f, my_format!(task), self.id, self.what, ""),
        }
    }
}

impl Display for SubTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.link {
            Some(l) => write!(f, my_format!(subtask), self.subtask_rank, self.what, l),
            None => write!(f, my_format!(subtask), self.subtask_rank, self.what, ""),
        }
    }
}

impl Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = NaiveDateTime::from_timestamp(self.finish_timestamp as i64, 0);
        let date: DateTime<Utc> = DateTime::from_utc(date, Utc);
        let date: DateTime<Local> = DateTime::from(date);
        // let date = date.format("%Y-%m-%d %H:%M:%S");
        let date = date.format("%Y-%m-%d");
        if let Some(l) = &self.link {
            write!(f, my_format!(history), date, self.what, l)
        } else {
            write!(f, my_format!(history), date, self.what, "")
        }
    }
}

pub fn prompt_finished_task() {
    println!(
        my_format!(id_history),
        "task_id", "date", "description", "link(optional)"
    );
}

pub fn prompt_task() {
    println!(my_format!(task), "task_id", "description", "link(optional)");
}

pub fn prompt_subtask(id: i32) {
    println!("subtask of {}", &id);
    println!(
        my_format!(subtask),
        "subtask_id", "description", "link(optional)"
    );
}
