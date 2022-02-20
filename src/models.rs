use diesel::Queryable;
use super::schema::{tasks, subtasks};

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
pub struct NewSubTask{
    pub what: String,
    pub link: Option<String>,
    pub task_id: i32,
    pub subtask_rank: i32,
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
