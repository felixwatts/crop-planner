mod constant;
mod harvest_plan;
mod rand;
mod solution;
mod solver;
mod variety;
mod bed;
mod params;
mod common;

use std::fs;
use crate::params::Params;
use std::convert::TryFrom;

fn main() {

    let params_str = fs::read_to_string("params.json")
        .expect("Failed to read params.json");
    let params_json = json::parse(&params_str).expect("Failed to parse params.json");
    let params = Params::try_from(&params_json).expect("Failed to parse params.json");

    let mut solver = crate::solver::Solver::new(params.clone());

    let mut best_score = -1000000;
    let mut gen = 0;

    loop {
        solver.step();

        let score = solver.get_best_score();

        if score > best_score {
            best_score = score;

            let best_solution = solver.get_best_solution();

            let harvest_plan = crate::solution::to_harvest_plan(&best_solution, &params);
            crate::harvest_plan::print_harvest_plan(&harvest_plan, &params);
        
            println!("gen: {} score: {}", gen, best_score);
        }

        gen += 1;
    }
}
