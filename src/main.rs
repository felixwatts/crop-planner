mod constant;
mod rand;
mod genome;
mod evolver;
mod variety;
mod bed;
mod params;
mod common;
mod cli;
mod repo;
mod bed_plan;
mod tasks;
mod cf;
mod evaluator;
mod formatter;

#[macro_use] extern crate lazy_static;

use structopt::StructOpt;
use crate::cli::*;
use crate::repo::Repo;
use std::io::prelude::*; 
use simple_error::*;                                                          

fn main() {

    let opt = crate::cli::Cli::from_args();

    let result = match opt.command {
        Cmd::Init => init(),
        Cmd::Solve => solve(),
        Cmd::Reset => reset(),
        Cmd::Print(params) => print(&params),
    };

    match result {
        Err(msg) => eprintln!("{}", msg),
        _ => ()
    };
}

fn init() -> Result<(), Box<dyn std::error::Error>> {
    let mut repo = Repo::new(&std::path::PathBuf::from("."));
    repo.init()
}

fn reset() -> Result<(), Box<dyn std::error::Error>> {
    let mut repo = Repo::new(&std::path::PathBuf::from("."));
    repo.load()?;
    repo.reset();
    repo.save()
}

fn print(cmd: &crate::cli::ParamsPrint) -> Result<(), Box<dyn std::error::Error>> {
    match &cmd.bed {
        Some(bed) => match cmd.week {
            Some(week) => print_bed_week(&bed, week),
            None => print_bed(&bed)
        }
        None => match cmd.week {
            Some(week) => print_week(week),
            None => print_solution()
        }
    }
}

fn print_bed(bed_name: &std::string::String) -> Result<(), Box<dyn std::error::Error>> {
    let repo = require_repo()?;
    let sol = repo.require_solution()?;
    
    let params = repo.get_params()?;
    let bed = require_bed(bed_name, &params)?;
    let evaluator = crate::evaluator::Evaluator::new(&params, &sol);
    let bed_plan = evaluator.get_bed_plan(bed);
    println!("{}", bed_plan);
    Ok(())
}

fn print_bed_week(bed: &std::string::String, week: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("Bed {}, Week {}", bed, week);
    Ok(()) // TODO
}

fn print_week(week: usize) -> Result<(), Box<dyn std::error::Error>> {
    let repo = require_repo()?;
    let planting_schedule = repo.require_solution()?;
    let params = repo.get_params()?;
    let evaluator = crate::evaluator::Evaluator::new(&params, &planting_schedule);
    let tasks = evaluator.get_tasks();    
    let week_instructions = tasks.get(week);

    println!("Tasks for week #{}", week);

    for t in week_instructions.iter() {
        println!("- {}", t);
    }

    Ok(())
}

fn print_solution() -> Result<(), Box<dyn std::error::Error>> {
    let repo = require_repo()?;
    let sol = repo.require_solution()?;
    let params = repo.get_params()?;
    let formatter = crate::formatter::Formatter::new(&params, &sol);
    println!("{}", &formatter);
    Ok(())
}

fn require_bed(name: &std::string::String, params: &crate::params::Params) -> Result<usize, Box<dyn std::error::Error>> {
    match params.get_bed(name) {
        Some(i) => Ok(i),
        None => bail!("Unknown bed")
    }
}

fn require_repo() -> Result<crate::repo::Repo, Box<dyn std::error::Error>> {
    let mut repo = Repo::new(&std::path::PathBuf::from("."));
    repo.load()?;
    Ok(repo)
}

fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let mut repo = Repo::new(&std::path::PathBuf::from("."));
    repo.load()?;

    repo.require_no_solution()?;

    let params = repo.get_params()?;

    let mut planner = crate::cf::CfPlanner::new(&params);
    let genes = planner.plan();

    repo.put_solution(genes)?;
    repo.save()?;

    println!("");

    Ok(())
}
