use crate::params::Params;
use crate::constant::{ NUM_BEDS, SEASON_LENGTH, SOLUTION_SIZE, WeekId, VarietyId };
use crate::harvest_plan::HarvestPlan;

pub type Solution = [VarietyId; SOLUTION_SIZE ];

pub fn new() -> Solution {
    return [ 0; SOLUTION_SIZE ];
}

pub fn to_harvest_plan(sol: &Solution, params: &Params) -> HarvestPlan {
    // build the harvest plan, which tells us how many units of each variety
    // are harvestable in each week
    let mut harvest_plan = std::iter::repeat(vec![ 0; params.varieties.len() ])
        .take(SEASON_LENGTH)
        .collect::<Vec<_>>();
    for bed in 0..NUM_BEDS {
        let mut week: WeekId = 0;
        for crop in 0..SEASON_LENGTH {
            let gene_id = bed * SEASON_LENGTH + crop;
            let variety_id = sol[gene_id];
            let variety = &params.varieties[variety_id];

            for growth_week in 0..variety.get_longevity() {
                let harvest_week = (week+growth_week) % SEASON_LENGTH;
                let harvest_units = variety.harvest_schedule[growth_week];
                harvest_plan[harvest_week][variety_id] += harvest_units;
            }

            week += variety.get_longevity();
        }
    }

    return harvest_plan;
}

// fn print_solution(sol: &Solution, params: &Params) {
//     for bed in 0..NUM_BEDS {
//         for crop in 0..SEASON_LENGTH {
//             let gene_id = bed * SEASON_LENGTH + crop;
//             let variety_id = sol[gene_id];
//             let variety = &params.varieties[variety_id];
//             print!("[{}] ", variety.name);
//         }
//         println!("");
//     }
// }