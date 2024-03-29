use anyhow::{Context, Result};
use diesel::associations::HasTable;
use diesel::dsl::max;
use diesel::{prelude::*, sql_query};
use diesel_migrations::embed_migrations;

use crate::create_connection;
use crate::models::{History, NewHistory, NewSubTask, NewTask, SubTask, Task};
use crate::schema::histories;
use crate::schema::subtasks::dsl::subtasks;
use crate::schema::tasks::dsl::*;

type TodoResult<T> = Result<T>;
type IDType = i32;

pub trait TaskDB {
    fn add_task(&mut self, task: NewTask) -> TodoResult<IDType>;
    fn add_subtask(&mut self, id: IDType, what: String, link: Option<String>) -> TodoResult<()>;
    fn get_task(&self, id: IDType) -> TodoResult<Option<Task>>;
    fn get_tasks(&self, pattern: Option<String>) -> TodoResult<Vec<Task>>;
    fn get_subtasks(&self, id: IDType) -> TodoResult<Vec<SubTask>>;
    fn get_finished(&self, last_n: u32) -> TodoResult<Vec<History>>;
    fn get_finished_within(&self, start_ts: u32, end_ts: u32) -> TodoResult<Vec<History>>;
    fn update_task_desc(&mut self, id: IDType, desc: String) -> TodoResult<()>;
    fn update_task_priority(&mut self, task_id: IDType, pri: i32) -> TodoResult<()>;
    fn remove_task(&mut self, id: IDType) -> TodoResult<()>;
    fn update_subtask_belongings(&mut self, task_id: IDType, new_task_id: IDType)
        -> TodoResult<()>;
    fn remove_subtask(&mut self, id: IDType, subtask_rank: i32) -> TodoResult<()>;
    fn finish_task(&mut self, id: IDType) -> TodoResult<()>;
}

pub struct TaskSqlite {
    conn: SqliteConnection,
}

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

impl TaskDB for TaskSqlite {
    fn add_task(&mut self, new_task: NewTask) -> TodoResult<IDType> {
        diesel::insert_into(tasks::table())
            .values(&new_task)
            .execute(&self.conn)
            .context("fail to add new task")?;
        // https://github.com/diesel-rs/diesel/issues/771
        let last_id = diesel::select(last_insert_rowid).get_result::<i32>(&self.conn)?;
        Ok(last_id)
    }

    fn add_subtask(
        &mut self,
        input_task_id: IDType,
        st_what: String,
        st_link: Option<String>,
    ) -> TodoResult<()> {
        // use i64 for count returned value
        let rank: i64 = subtasks
            .count()
            .filter(crate::schema::subtasks::dsl::task_id.eq(input_task_id))
            .first(&self.conn)?;
        let new_subtask = NewSubTask {
            what: st_what,
            link: st_link,
            task_id: input_task_id,
            subtask_rank: 1 + rank as i32,
        };
        diesel::insert_into(subtasks::table())
            .values(&new_subtask)
            .execute(&self.conn)
            .expect("fail to add new subtask");
        Ok(())
    }

    fn get_task(&self, task_id: i32) -> TodoResult<Option<Task>> {
        let task = tasks
            .find(task_id)
            .first::<Task>(&self.conn)
            .context(format!("data store failed to find task {}", task_id))?;
        Ok(Some(task))
    }

    fn get_subtasks(&self, input_task_id: IDType) -> TodoResult<Vec<SubTask>> {
        // return subtasks associated with a task
        let task = tasks
            .find(input_task_id)
            .first::<Task>(&self.conn)
            .expect("Task not found!");
        let results = SubTask::belonging_to(&task)
            .load::<SubTask>(&self.conn)
            .context("fail to find subtask")?;
        Ok(results)
    }

    fn get_tasks(&self, pattern: Option<String>) -> TodoResult<Vec<Task>> {
        if let Some(pattern) = pattern {
            let results = tasks
                .filter(what.like(format!("%{}%", pattern)))
                .load::<Task>(&self.conn)?;
            Ok(results)
        } else {
            Ok(tasks.load::<Task>(&self.conn)?)
        }
    }

    fn get_finished(&self, last_n: u32) -> TodoResult<Vec<History>> {
        Ok(histories::dsl::histories
            .order_by(histories::dsl::finish_timestamp.desc())
            .limit(last_n as i64)
            .load::<History>(&self.conn)?)
    }

    fn get_finished_within(&self, start_ts: u32, end_ts: u32) -> TodoResult<Vec<History>> {
        Ok(histories::dsl::histories
            .filter(histories::dsl::finish_timestamp.ge(start_ts as i32))
            .filter(histories::dsl::finish_timestamp.lt(end_ts as i32))
            .order_by(histories::dsl::finish_timestamp.desc())
            .load::<History>(&self.conn)?)
    }

    fn remove_task(&mut self, task_id: IDType) -> TodoResult<()> {
        let rows_affected = diesel::delete(tasks.filter(id.eq_all(task_id)))
            .execute(&self.conn)
            .context(format!("data store failed to remove task {}", task_id))?;
        if rows_affected == 0 {
            println!("task {} not found!", task_id);
        }
        self.try_reset_id("tasks")?;
        Ok(())
    }

    fn finish_task(&mut self, task_id: IDType) -> TodoResult<()> {
        let task = self
            .get_task(task_id)
            .context("finish task: fail to find task")?
            .unwrap();
        self.remove_task(task_id)?;
        let new_history = NewHistory {
            what: task.what,
            link: task.link,
            finish_timestamp: chrono::Utc::now().timestamp() as i32,
        };
        let rows_affected = diesel::insert_into(histories::dsl::histories::table())
            .values(&new_history)
            .execute(&self.conn)?;
        if rows_affected == 0 {
            println!("fail to finish task {}!", task_id);
        }
        Ok(())
    }

    fn update_task_desc(&mut self, task_id: IDType, desc: String) -> TodoResult<()> {
        diesel::update(tasks.filter(id.eq(task_id)))
            .set(what.eq(desc))
            .execute(&self.conn)?;
        Ok(())
    }

    fn update_task_priority(&mut self, task_id: IDType, pri: i32) -> TodoResult<()> {
        diesel::update(tasks.filter(id.eq(task_id)))
            .set(priority.eq(pri))
            .execute(&self.conn)?;
        Ok(())
    }

    fn update_subtask_belongings(
        &mut self,
        old_task_id: IDType,
        new_task_id: IDType,
    ) -> TodoResult<()> {
        use crate::schema::subtasks::dsl::task_id;
        diesel::update(subtasks.filter(task_id.eq_all(old_task_id)))
            .set(task_id.eq(new_task_id))
            .execute(&self.conn)?;
        Ok(())
    }

    fn remove_subtask(&mut self, input_task_id: IDType, input_subtask_rank: i32) -> TodoResult<()> {
        use crate::schema::subtasks::dsl::{subtask_rank, task_id};
        let rows_affected = diesel::delete(
            subtasks.filter(
                task_id
                    .eq_all(input_task_id)
                    .and(subtask_rank.eq_all(input_subtask_rank)),
            ),
        )
        .execute(&self.conn)?;
        if rows_affected == 0 {
            println!(
                "subtask {} for task {} not found!",
                input_subtask_rank, input_task_id
            );
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
            subtasks
                .select(max(crate::schema::subtasks::dsl::id))
                .first::<Option<i32>>(&self.conn)?
        }
        .unwrap_or(0);
        let query = sql_query(format!(
            "UPDATE `sqlite_sequence` SET `seq`={} WHERE `name`='{}'",
            count, table_name
        ));
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
