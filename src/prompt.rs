pub fn prompt_task() {
    println!(
        "{0: <10} {1: <30} {2: <10}",
        "task_id", "description", "link(optional)"
    );
}

pub fn prompt_finished_task() {
    println!(
        "{0: <10} {1: <20} {2: <20} {3: <10}",
        "task_id", "description", "link(optional)", "finished date"
    );
}

pub fn prompt_subtask(id: i32) {
    println!("subtask of {}", &id);
    println!(
        "{0: <10} {1: <30} {2: <10}",
        "subtask_id", "description", "link(optional)"
    );
}
