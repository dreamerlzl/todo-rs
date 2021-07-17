use std::env;
use log::debug;
use env_logger::Env;

mod taskdb;

use clap::{Clap, AppSettings};

use crate::taskdb::{prompt_subtask, prompt_task};

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts{
    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long)]
    task_id: Option<u32>,  // the task id

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand{
    List(List),
    End(End),
    Add(Add),
}

#[derive(Debug)]
#[derive(Clap)]
struct Add{
    desc: String,
    #[clap(short, long)]
    link: Option<String>,
}


#[derive(Debug)]
#[derive(Clap)]
struct List{
    pattern: Option<String>,
}

#[derive(Debug)]
#[derive(Clap)]
struct End{
    id_or_order: u32,
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

    let db_path = env::var("TODO_DB")?;
    let mut db = taskdb::open(&db_path)?;

    match opts.subcmd {
        SubCommand::Add(add) => {
            // add a new task
            debug!("{:?}", add);
            if let Some(id) = opts.task_id {
                db.add_subtask(id, add.desc, add.link)?;
            } else {
                db.add_task(add.desc, add.link)?;
            }
        }
        SubCommand::List(list) => {
            debug!("{:?}", list);
            if let Some(id) = opts.task_id {
                let subtasks = db.get_subtasks(id)?;
                prompt_subtask(id);
                subtasks
                .iter()
                .for_each(|subtask| println!("{}", subtask));
                // log output
            } else {
                let tasks = db.get_tasks(list.pattern)?;
                prompt_task();
                tasks
                .iter()
                .for_each(|t| println!("{}", t));
            }
        }
        SubCommand::End(end) => {
            debug!("{:?}", end);
            if let Some(t) = opts.task_id {
                let order = end.id_or_order;
                db.remove_subtask(t, order)?;
            } else {
                let id = end.id_or_order;
                db.remove_task(id)?;
            }
        }
    }
    Ok(())
}
