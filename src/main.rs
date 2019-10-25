mod constant;
mod harvest_plan;
mod rand;
mod genome;
mod solver;
mod variety;
mod bed;
mod params;
mod common;
mod cli;
mod repo;

use crate::constant::SEASON_LENGTH;
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
        Cmd::Print(params) => print(&params)
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
    match cmd.bed {
        Some(bed) => match cmd.week {
            Some(week) => print_bed_week(bed, week),
            None => print_bed(bed)
        }
        None => match cmd.week {
            Some(week) => print_week(week),
            None => print_solution()
        }
    }
}

fn print_bed(bed: usize) -> Result<(), Box<dyn std::error::Error>> {
    let repo = require_repo()?;
    let sol = repo.require_solution()?;
    let params = repo.get_params()?;
    println!("Bed {}", bed);
    println!("{:<9}{:<9}", "Week", "Variety");
    for week in 0..SEASON_LENGTH {
        let variety = sol.get_variety(bed, week);
        if variety != 0 {
            let variety_name = &params.varieties[variety].name;
            println!("{:<9}{:<9}", week, variety_name);
        }
    }
    let stats = sol.get_bed_stats(bed, &params);
    println!("Utilization: {:.0}%", stats.utilization() * 100.0);
    Ok(())
}

fn print_bed_week(bed: usize, week: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("Bed {}, Week {}", bed, week);
    Ok(()) // TODO
}

fn print_week(week: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("Week {}", week);
    Ok(()) // TODO
}

fn print_solution() -> Result<(), Box<dyn std::error::Error>> {
    let repo = require_repo()?;
    let sol = repo.require_solution()?;
    let params = repo.get_params()?;
    let harvest_plan = sol.to_harvest_plan(&params);
    crate::harvest_plan::print_harvest_plan(&harvest_plan, &params);
    Ok(())
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
    let mut solver = solver::Solver::new(&params);

    let mut best_score = std::i32::MIN;
    let mut gen = 0;
    let mut gens_without_improvement = 0;

    loop {

        if gens_without_improvement > 1000 {
            break;
        }

        solver.step();

        let score = solver.get_best_score();

        if score > best_score {
            best_score = score;
            gens_without_improvement = 0;

            // let best_solution = solver.get_best_solution();
            // let harvest_plan = crate::solution::to_harvest_plan(&best_solution, &params);
            // crate::harvest_plan::print_harvest_plan(&harvest_plan, &params);
            // println!("gen: {} score: {}", gen, best_score);

            print!(".");
            std::io::stdout().flush();
            
        } else {
            gens_without_improvement += 1;
        }

        gen += 1;
    };

    repo.put_solution(solver.get_best_solution());
    repo.save()?;

    println!("");

    Ok(())
}
