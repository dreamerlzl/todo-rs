use assert_cmd::assert::Assert;
use assert_cmd::Command;
use predicates::str::contains;

const BINARY_NAME: &'static str = "todo";
const TODO_DB: &'static str = "./todo.db";

#[test]
fn cli_no_args() {
    Command::cargo_bin(BINARY_NAME).unwrap().assert().failure();
}

#[test]
fn cli_list() {
    add_task("aria", None);
    list_tasks_contains("aria");
    add_task("yokohama kaidashi kikou", Some("test"));
    list_tasks_contains("aria");
    list_tasks_contains("kaidashi");
    list_tasks_contains("test");
    finish_task(1, true);
    finish_task(2, true);
}

#[test]
fn cli_tidy() {
    add_task("a", None);
    add_task("b", None);
    add_task("c", None);
    finish_task(2, true);
    // after finish 2, 1 and 3 left
    tidy();
    // after tidy, only 1 and 2! no 3
    finish_task(3, false);
    finish_task(1, true);
    finish_task(2, true);
}

fn command_assert(args: &[&str]) -> Assert {
    Command::cargo_bin(BINARY_NAME)
        .unwrap()
        .env("TODO_DB", TODO_DB)
        .args(args)
        .assert()
}

fn list_tasks_contains(pattern: &str) {
    command_assert(&["list"]).stdout(contains(pattern));
}

fn add_task(what: &str, link: Option<&str>) {
    if let Some(l) = link {
        Command::cargo_bin(BINARY_NAME)
            .unwrap()
            .args(&["add", what, "-l", l])
            .env("TODO_DB", TODO_DB)
            .assert()
            .success();
    } else {
        Command::cargo_bin(BINARY_NAME)
            .unwrap()
            .env("TODO_DB", TODO_DB)
            .args(&["add", what])
            .assert()
            .success();
    }
}

fn finish_task(id: i32, success: bool) {
    let a = command_assert(&["fin", &id.to_string()]);
    if success {
        a.success();
    } else {
        a.failure();
    }
}

fn tidy() {
    command_assert(&["tidy"]).success();
}

fn clean() {
    // command_assert()
}
