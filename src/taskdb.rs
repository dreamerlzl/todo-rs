use diesel::associations::HasTable;
use anyhow::{Context, Result};
use diesel::{prelude::*, sql_query};
use diesel::dsl::max;
use diesel_migrations::embed_migrations;

use crate::create_connection;
use crate::schema::subtasks::dsl::subtasks;
use crate::schema::tasks::dsl::*;
use crate::models::{SubTask, Task, NewTask, NewSubTask};

type TodoResult<T> = Result<T>;
type IDType = i32;

pub trait TaskDB {
    fn add_task(&mut self, what: String, link: Option<String>) -> TodoResult<()>;
    fn add_subtask(&mut self, id: IDType, what: String, link: Option<String>) -> TodoResult<()>;
    fn get_task(&self, id: IDType) -> TodoResult<Option<Task>>;
    fn get_tasks(&self, pattern: Option<String>) -> TodoResult<Vec<Task>>;
    fn get_subtasks(&self, id: IDType) -> TodoResult<Vec<SubTask>>;
    fn remove_task(&mut self, id: IDType) -> TodoResult<()>;
    fn remove_subtask(&mut self, id: IDType, subtask_rank: i32) -> TodoResult<()>;
}

pub fn print_subtasks(subtasks_to_print: Vec<SubTask>, indent_level: usize) -> Vec<String> {
    let indent = "  ".repeat(indent_level);
    subtasks_to_print
        .iter()
        .map(|st| {
            if let Some(l) = &st.link {
                format!("{0}{1: <10} {2: <30} {3: <10}", indent, st.id, st.what, l,)
            } else {
                format!("{0}{1: <10} {2: <30}", indent, st.id, st.what,)
            }
        })
        .collect()
}

pub struct TaskSqlite {
    conn: SqliteConnection,
}

impl TaskDB for TaskSqlite {
    fn add_task(&mut self, task_what: String, task_link: Option<String>) -> TodoResult<()> {
        let new_task = NewTask{what: task_what, link: task_link};
        diesel::insert_into(tasks::table()).values(&new_task).execute(&self.conn).expect("fail to add new task");
        Ok(())
    }

    fn add_subtask(&mut self, input_task_id: IDType, st_what: String, st_link: Option<String>) -> TodoResult<()> {
        // use i64 for count returned value
        let rank: i64 = subtasks.count().filter(
            crate::schema::subtasks::dsl::task_id.eq(input_task_id)).first(&self.conn)?;
        let new_subtask = NewSubTask{
            what: st_what,
            link: st_link,
            task_id: input_task_id,
            subtask_rank: rank as i32,
        };
        diesel::insert_into(subtasks::table()).values(&new_subtask).execute(&self.conn).expect("fail to add new subtask");
        Ok(())
    }

    fn get_task(&self, task_id: i32) -> TodoResult<Option<Task>> {
        let task = tasks.find(task_id).first::<Task>(&self.conn)?;
        Ok(Some(task))
    }

    fn get_subtasks(&self, input_task_id: IDType) -> TodoResult<Vec<SubTask>> {
        // return subtasks associated with a task
        let task = tasks.find(input_task_id).first::<Task>(&self.conn).expect("Task not found!");
        let results = SubTask::belonging_to(&task).load::<SubTask>(&self.conn).context("fail to find subtask")?;
        Ok(results)
    }

    fn get_tasks(&self, pattern: Option<String>) -> TodoResult<Vec<Task>> {
        if let Some(pattern) = pattern {
            let results = tasks.filter(what.like(format!("%{}%", pattern))).load::<Task>(&self.conn)?;
            Ok(results)
        } else {
            Ok(tasks.load::<Task>(&self.conn)?)
        }
    }

    fn remove_task(&mut self, task_id: IDType) -> TodoResult<()> {
        let rows_affected = diesel::delete(tasks.filter(id.eq_all(task_id))).execute(&self.conn)?;
        if rows_affected == 0 {
            println!("task {} not found!", task_id);
        }
        self.try_reset_id("tasks")?;
        Ok(())
    }

    fn remove_subtask(&mut self, input_task_id: IDType, input_subtask_rank: i32) -> TodoResult<()> {
        use crate::schema::subtasks::dsl::{task_id, subtask_rank};
        let rows_affected = diesel::delete(
            subtasks.filter(task_id.eq_all(input_task_id).and(subtask_rank.eq_all(input_subtask_rank)))
            ).execute(&self.conn)?;
        if rows_affected == 0 {
            println!("subtask{} for task {} not found!", input_subtask_rank, input_task_id);
        }
        self.try_reset_id("subtasks")?;
        Ok(())
    }
}

impl TaskSqlite {
    fn try_reset_id(&mut self, table_name: &str) -> TodoResult<()> {
        let count = if table_name == "tasks" {
            tasks.select(max(id)).first(&self.conn)?
        } else {
            subtasks.select(max(crate::schema::subtasks::dsl::id)).first::<Option<i32>>(&self.conn)?
        }.unwrap_or(0);
        let query = sql_query(format!("UPDATE `sqlite_sequence` SET `seq`={} WHERE `name`='{}'", count, table_name));
        query.execute(&self.conn).expect("fail to reset id");
        Ok(())
    }
}

//use diesel_migrations::embed_migrations;

embed_migrations!();

pub fn open(db_url: &str) -> TodoResult<Box<dyn TaskDB>> {
    let conn = create_connection(db_url.to_owned());
    embedded_migrations::run(&conn)?;
    Ok(Box::new(TaskSqlite { conn }))
}
