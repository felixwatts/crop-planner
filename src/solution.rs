use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::constant::{ NUM_BEDS, NUM_VARIETIES, SEASON_LENGTH, SOLUTION_SIZE, WeekId, VarietyId };
use crate::variety::Varieties;
use crate::harvest_plan::HarvestPlan;

pub type Solution = [VarietyId; SOLUTION_SIZE ];

pub fn new() -> Solution {
    return [ 0; SOLUTION_SIZE ];
}

pub fn to_harvest_plan(sol: &Solution, varieties: &Varieties) -> HarvestPlan {
    // build the harvest plan, which tells us how many units of each variety
    // are harvestable in each week
    let mut harvest_plan = [ [ 0; NUM_VARIETIES ]; SEASON_LENGTH ];
    for bed in 0..NUM_BEDS {
        let mut week: WeekId = 0;
        for crop in 0..SEASON_LENGTH {
            let gene_id = bed * SEASON_LENGTH + crop;
            let variety_id = sol[gene_id];
            let variety = &varieties[variety_id];

            for growth_week in 0..variety.get_longevity() {
                let harvest_week = (week+growth_week) % SEASON_LENGTH;
                let harvest_units = variety.harvest_schedule[growth_week];
                // if harvest_week < SEASON_LENGTH {
                harvest_plan[harvest_week][variety_id] += harvest_units;
                // }
            }

            week += variety.get_longevity();
        }
    }

    return harvest_plan;
}

fn print_solution(sol: &Solution, varieties: &Varieties) {
    for bed in 0..NUM_BEDS {
        for crop in 0..SEASON_LENGTH {
            let gene_id = bed * SEASON_LENGTH + crop;
            let variety_id = sol[gene_id];
            let variety = &varieties[variety_id];
            print!("[{}] ", variety.name);
        }
        println!("");
    }
}

fn write_solution(sol: &Solution, varieties: &Varieties) {
    let path = Path::new("sol.csv");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create file: {}", why.description()),
        Ok(file) => file,
    };

    
    for bed in 0..NUM_BEDS {
        write!(file, "Bed{},", bed);
    }
    writeln!(file, "");

    for crop in 0..SEASON_LENGTH {
        for bed in 0..NUM_BEDS {
            let gene_id = bed * SEASON_LENGTH + crop;
            let variety_id = sol[gene_id];
            let variety = &varieties[variety_id];
            write!(file, "{},", variety.name);
        }
        writeln!(file, "");
    }

    file.flush();
}

