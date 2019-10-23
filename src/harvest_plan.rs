use crate::constant::{ SEASON_LENGTH, HarvestableUnits };
use crate::params::Params;

pub type HarvestPlan = Vec<Vec<HarvestableUnits>>;

pub fn print_harvest_plan(harvest_plan: &HarvestPlan, params: &Params) {
    println!("");
    for v in 0..params.varieties.len() {
        print!("{:<9}", params.varieties[v].name);
        for w in 0..SEASON_LENGTH {
            print!("{:>3}", harvest_plan[w][v]);
        }
        println!("");
    }
}