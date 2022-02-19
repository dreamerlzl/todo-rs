use anyhow::Context;
use env_logger::Env;
use std::env;

mod taskdb;

use clap::{Parser, Subcommand};

use crate::taskdb::{print_subtasks, prompt_subtask, prompt_task};

#[derive(Parser, Debug)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long)]
    task_id: Option<u32>, // the task id

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    List {
        pattern: Option<String>,
    },
    End {
        id_or_order: u32,
    },
    Add {
        desc: String,
        #[clap(short, long)]
        link: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let opts: Opts = Opts::parse();

    if opts.verbose {
        println!("verbose!");
    }

    let db_path = env::var("TODO_DB").context("please define environment variable TODO_DB")?;
    let mut db = taskdb::open(&db_path)?;

    match opts.subcmd {
        SubCommand::Add { desc, link } => {
            // add a new task
            if let Some(id) = opts.task_id {
                db.add_subtask(id, desc, link)?;
            } else {
                db.add_task(desc, link)?;
            }
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
        SubCommand::End { id_or_order } => {
            if let Some(t) = opts.task_id {
                let order = id_or_order;
                db.remove_subtask(t, order)?;
            } else {
                let id = id_or_order;
                db.remove_task(id)?;
            }
        }
    }
    Ok(())
}
