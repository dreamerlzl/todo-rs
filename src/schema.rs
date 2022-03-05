table! {
    histories (id) {
        id -> Integer,
        what -> Text,
        link -> Nullable<Text>,
        finish_timestamp -> Integer,
    }
}

table! {
    subtasks (id) {
        id -> Integer,
        what -> Text,
        link -> Nullable<Text>,
        subtask_rank -> Integer,
        task_id -> Integer,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        what -> Text,
        link -> Nullable<Text>,
    }
}

joinable!(subtasks -> tasks (task_id));

allow_tables_to_appear_in_same_query!(
    histories,
    subtasks,
    tasks,
);
