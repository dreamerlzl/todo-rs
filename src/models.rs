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

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.link {
            Some(l) => write!(f, "{0: <10} {1: <30} {2: <10}", self.id, self.what, l),
            None => write!(f, "{0: <10} {1: <30}", self.id, self.what),
        }
    }
}

impl Display for SubTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.link {
            Some(l) => write!(
                f,
                "{0: <10} {1: <30} {2: <10}",
                self.subtask_rank, self.what, l
            ),
            None => write!(f, "{0: <10} {1: <30}", self.subtask_rank, self.what),
        }
    }
}

impl Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = NaiveDateTime::from_timestamp(self.finish_timestamp as i64, 0);
        let date: DateTime<Utc> = DateTime::from_utc(date, Utc);
        let date: DateTime<Local> = DateTime::from(date);
        let date = date.format("%Y-%m-%d %H:%M:%S");
        if let Some(l) = &self.link {
            write!(f, "{0: <20} {1: <20} {2: <10}", self.what, l, date)
        } else {
            write!(f, "{0: <20} {1: <20} {2: <10}", self.what, "", date)
        }
    }
}
