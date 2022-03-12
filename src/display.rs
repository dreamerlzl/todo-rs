use std::fmt::Display;

use chrono::{DateTime, Local, NaiveDateTime, Utc};

use super::models::{History, SubTask, Task};

macro_rules! my_format {
    (task) => {
        "{0: <10} {1: <50} {2: <10}"
    };
    (subtask) => {
        "{0: <10} {1: <50} {2: <10}"
    };
    (indent_subtask) => {
        "{0}{1: <10} {2: <30} {3: <10}"
    };
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

pub fn print_subtasks(subtasks_to_print: Vec<SubTask>, indent_level: usize) -> Vec<String> {
    let indent = "  ".repeat(indent_level);
    subtasks_to_print
        .iter()
        .map(|st| {
            if let Some(l) = &st.link {
                format!(my_format!(indent_subtask), indent, st.id, st.what, l,)
            } else {
                format!(my_format!(indent_subtask), indent, st.id, st.what, "")
            }
        })
        .collect()
}
