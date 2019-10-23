use crate::params::Params;
use crate::constant::{ SEASON_LENGTH, WeekId, VarietyId };
use crate::harvest_plan::HarvestPlan;

pub type Solution = Vec<VarietyId>;

pub fn new(params: &Params) -> Solution {
    vec![ 0; params.genome_size() ]
}

pub fn to_harvest_plan(sol: &Solution, params: &Params) -> HarvestPlan {
    // build the harvest plan, which tells us how many units of each variety
    // are harvestable in each week
    let mut harvest_plan = std::iter::repeat(vec![ 0; params.varieties.len() ])
        .take(SEASON_LENGTH)
        .collect::<Vec<_>>();
    for bed in 0..params.num_beds() {
        // let mut week: WeekId = 0;
        for planting_week in 0..SEASON_LENGTH {
            let gene_id = bed * SEASON_LENGTH + planting_week;
            let variety_id = sol[gene_id];
            let variety = &params.varieties[variety_id];

            for growth_week in 1..variety.get_longevity() {
                let harvest_week = (planting_week+growth_week) % SEASON_LENGTH;


                if sol[harvest_week] != 0 {
                    break;
                }

                let harvest_units = variety.harvest_schedule[growth_week];
                harvest_plan[harvest_week][variety_id] += harvest_units;
            }

            // week += variety.get_longevity();
        }
    }

    return harvest_plan;
}

pub fn print_solution(sol: &Solution, params: &Params) {
    for bed in 0..params.beds.len() {
        for crop in 0..SEASON_LENGTH {
            let gene_id = bed * SEASON_LENGTH + crop;
            let variety_id = sol[gene_id];
            let variety = &params.varieties[variety_id];
            print!("[{}] ", variety.name);
        }
        println!("");
    }
}