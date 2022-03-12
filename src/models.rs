use super::schema::{histories, subtasks, tasks};
use diesel::Queryable;

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
    pub task_id: i32,
    pub subtask_rank: i32,
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
