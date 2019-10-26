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
mod bed_plan;
mod instructions;
mod phenome;

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
        Cmd::Instruct(params) => instruct(&params)
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
    let genome = crate::genome::Genome::from_genes(sol, &params);
    let phenome = genome.to_phenome();
    let bed_plan = phenome.get_bed_plan(bed);
    println!("{}", bed_plan);
    Ok(())
}

fn print_bed_week(bed: &std::string::String, week: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("Bed {}, Week {}", bed, week);
    Ok(()) // TODO
}

fn print_week(week: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("Week {}", week);
    Ok(()) // TODO
}

fn print_solution() -> Result<(), Box<dyn std::error::Error>> {
    let repo = require_repo()?;
    let genes = repo.require_solution()?;
    let params = repo.get_params()?;
    let genome = crate::genome::Genome::from_genes(genes, &params);
    let harvest_plan = genome.to_phenome().get_harvest_plan();
    crate::harvest_plan::print_harvest_plan(&harvest_plan, &params);
    Ok(())
}

fn instruct(params_instruct: &ParamsInstruct) -> Result<(), Box<dyn std::error::Error>> {
    let repo = require_repo()?;
    let genes = repo.require_solution()?;
    let params = repo.get_params()?;
    let genome = crate::genome::Genome::from_genes(genes, &params);
    let phenome = genome.to_phenome();
    let instructions = phenome.get_instructions();    
    let week_instructions = instructions.get(params_instruct.week);

    println!("Tasks for week #{}", params_instruct.week);

    for t in week_instructions.iter() {
        println!("- {}", t);
    }

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
    let mut solver = solver::Solver::new(&params);

    let mut best_score = std::i32::MIN;
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

            print!(".");
            std::io::stdout().flush()?;
            
        } else {
            gens_without_improvement += 1;
        }
    };

    repo.put_solution(solver.get_best_solution())?;
    repo.save()?;

    println!("");

    Ok(())
}
