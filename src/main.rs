use std::io::Write;
use std::process::Command;
use std::{env, fs};

use anyhow::Context;
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use tempfile::NamedTempFile;

use todo::display::{print_subtasks, prompt_finished_task, prompt_subtask, prompt_task};
use todo::taskdb::open;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long)]
    task_id: Option<i32>, // the task id

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    List {
        pattern: Option<String>,
    },
    Del {
        id_or_order: i32,
    },
    Fin {
        id_or_order: i32,
    },
    Add {
        desc: String,
        #[clap(short, long)]
        link: Option<String>,
    },
    Update {
        id_or_order: i32,
    },
    Tidy,
    Note {
        desc: String,
        #[clap(short, long)]
        link: Option<String>,
    },
    History {
        #[clap(short, long)]
        n: Option<u32>,

        // to search tasks finished with a date range
        // date format: 2012-1-3, 2021-12-29, etc.
        #[clap(short, long)]
        start_date: Option<String>,

        #[clap(short, long)]
        end_date: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let env = Env::default()
    //     .filter_or("MY_LOG_LEVEL", "info")
    //     .write_style_or("MY_LOG_STYLE", "always");

    // env_logger::init_from_env(env);

    let opts: Opts = Opts::parse();

    let db_path = env::var("TODO_DB").context("please define environment variable TODO_DB")?;
    let mut db = open(&db_path)?;

    match opts.subcmd {
        SubCommand::Add { desc, link } => {
            // add a new task
            if let Some(id) = opts.task_id {
                db.add_subtask(id, desc, link)?;
            } else {
                db.add_task(desc, link)?;
            }
        }
        SubCommand::Update { id_or_order } => {
            // create a tempfile with current desc as the content
            // spawn vi to edit the tempfile
            // and update the current desc with the final file content
            if let Some(task) = db.get_task(id_or_order)? {
                let mut current_desc = NamedTempFile::new()?;
                current_desc.write_all(task.what.as_bytes())?;
                let path = current_desc.path();
                Command::new("vi")
                    .arg(path)
                    .status()
                    .expect("fail to use vi to update desc");
                let new_desc: String = fs::read_to_string(path)
                    .expect("fail to read new desc")
                    .trim()
                    .to_string();
                db.update_task_desc(id_or_order, new_desc)?;
            } else {
                println!("no such task {id_or_order}!");
            }
        }
        SubCommand::Note { desc, link } => {
            let task_id = db.add_task(desc, link)?;
            db.finish_task(task_id)?;
        }
        SubCommand::List { pattern } => {
            if let Some(id) = opts.task_id {
                let subtasks = db.get_subtasks(id)?;
                prompt_subtask(id);
                subtasks.iter().for_each(|subtask| println!("{}", subtask));
                // log output
            } else {
                let tasks = db.get_tasks(pattern)?;
                prompt_task();
                tasks.iter().for_each(|t| {
                    println!("{}", t);
                    print_subtasks(db.get_subtasks(t.id).unwrap(), 1)
                        .iter()
                        .for_each(|st| {
                            println!("{}", st);
                        });
                });
            }
        }
        SubCommand::Tidy => {
            let tasks = db.get_tasks(None)?;
            for t in tasks.iter() {
                db.remove_task(t.id)?;
            }
            for t in tasks {
                db.add_task(t.what, t.link)?;
            }
        }
        SubCommand::History {
            n: last_n,
            start_date,
            end_date,
        } => {
            let finished_tasks = if let Some(last_n) = last_n {
                db.get_finished(last_n)?
            } else {
                // let start_ts = start_date.map(|sd| NaiveDate::parse_from_str(&sd, "%Y-%m-%d")?);
                let start_ts = if let Some(start_date) = start_date {
                    let date = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
                        .expect("fail to parse start time");
                    date.and_hms(0, 0, 0).timestamp() as u32
                } else {
                    0
                };
                let end_ts = if let Some(end_date) = end_date {
                    let date = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
                        .expect("fail to parse end time");
                    date.and_hms(0, 0, 0).timestamp() as u32
                } else {
                    chrono::Utc::now().timestamp() as u32
                };
                db.get_finished_within(start_ts, end_ts)?
            };
            prompt_finished_task();
            finished_tasks
                .into_iter()
                .enumerate()
                .for_each(|(i, finished_task)| {
                    println!("{: <10} {}", i, finished_task);
                });
        }
        SubCommand::Del { id_or_order } => {
            if let Some(t) = opts.task_id {
                let order = id_or_order;
                db.remove_subtask(t, order)?;
            } else {
                let id = id_or_order;
                db.remove_task(id)?;
            }
        }
        SubCommand::Fin { id_or_order } => {
            if let Some(t) = opts.task_id {
                // a finish of subtask would not be added into history
                db.remove_subtask(t, id_or_order)?;
            } else {
                db.finish_task(id_or_order)?;
            }
        }
    }
    Ok(())
}
