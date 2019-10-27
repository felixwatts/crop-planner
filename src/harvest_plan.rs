use std::fmt::Display;
use crate::constant::VarietyId;
use crate::constant::WeekId;
use crate::constant::{ SEASON_LENGTH, HarvestableUnits };
use crate::params::Params;

// Represents the harvestable units of each variety in each week implied by a given crop plan
// Used as part of the fitness function phenome.score(), and to display the crop plan in the console
pub struct HarvestPlan<'a> {
    params: &'a Params,
    data: Vec<Vec<HarvestableUnits>>
}

impl<'a> HarvestPlan<'a> {
    pub fn new(params: &'a Params) -> Self {
        let data = std::iter::repeat(vec![ 0; params.varieties.len() ])
            .take(SEASON_LENGTH)
            .collect::<Vec<_>>();
        HarvestPlan{ 
            params: params,
            data: data
        }
    }

    pub fn add(&mut self, week: WeekId, variety: VarietyId, harvestable_units: HarvestableUnits) {
        self.data[week][variety] += harvestable_units
    }

    pub fn get(&self, week: WeekId, variety: VarietyId) -> HarvestableUnits {
        self.data[week][variety]
    }
}

impl Display for HarvestPlan<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for v in 0..self.params.varieties.len() {
            write!(f, "{:<18}", self.params.varieties[v].name)?;
            for w in 0..SEASON_LENGTH {
                write!(f, "{:>3}", self.get(w, v))?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}