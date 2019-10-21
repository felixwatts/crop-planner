use crate::constant::{ NUM_VARIETIES, SEASON_LENGTH, HarvestableUnits };
use crate::variety::Varieties;

pub type HarvestPlan = [ [ HarvestableUnits; NUM_VARIETIES ]; SEASON_LENGTH ];

pub fn print_harvest_plan(harvest_plan: &HarvestPlan, varieties: &Varieties) {
    println!("");
    for v in 0..NUM_VARIETIES {
        print!("{:<9}", varieties[v].name);
        for w in 0..SEASON_LENGTH {
            print!("{:>3}", harvest_plan[w][v]);
        }
        println!("");
    }
}