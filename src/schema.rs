// @generated automatically by Diesel CLI.

diesel::table! {
    histories (id) {
        id -> Integer,
        what -> Text,
        link -> Nullable<Text>,
        finish_timestamp -> Integer,
    }
}

diesel::table! {
    subtasks (id) {
        id -> Integer,
        what -> Text,
        link -> Nullable<Text>,
        subtask_rank -> Integer,
        task_id -> Integer,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        what -> Text,
        link -> Nullable<Text>,
        priority -> Integer,
    }
}

diesel::joinable!(subtasks -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(histories, subtasks, tasks,);
