use crate::constant::NUM_BOXES;
use crate::constant::SEASON_LENGTH;
use crate::harvest_plan::HarvestPlan;
use crate::bed_plan::BedPlan;
use crate::tasks::Tasks;
use crate::params::Params;
use std::convert::TryInto;

// Represents the real world expression (crop plan) of the genome
// Provides methods to evaluate and explore the plan
pub struct Phenome<'a> {
    genes: &'a Vec<usize>,
    params: &'a Params
}

impl<'a> Phenome<'_> {
    pub fn new(genes: &'a Vec<usize>, params: &'a Params) -> Phenome<'a> {
        Phenome{
            genes: genes,
            params: params
        }
    }

    pub fn get_tasks(&self) -> Tasks {
        let mut result = Tasks::new();
        for bed in 0..self.params.beds.len() {
            let plan = self.get_bed_plan(bed);
            plan.write_instructions(&mut result);
        }
        result
    }

    // This is the fitness function for the evolutionary algorithm
    pub fn score(&self) -> i32 {
        // factor to minimize plantings
        let num_plantings: i32 = self.genes.iter().filter(|&x| *x != 0).count().try_into().unwrap();
        let mut score = -num_plantings;

        let harvest_plan = self.get_harvest_plan();

        // factor to match harvest to target harvest
        for week in 0..SEASON_LENGTH {
            for variety_id in 0..self.params.varieties.len() {
                let harvestable_units = harvest_plan.get(week, variety_id);
                score -= (harvestable_units - NUM_BOXES).abs();
            }
        }

        score
    }

    pub fn get_bed_plan(&'a self, bed: usize) -> BedPlan<'a> {
        BedPlan::new(bed, &self.genes, self.params)
    }

    pub fn get_harvest_plan(&self) -> HarvestPlan {
        let mut harvest_plan = HarvestPlan::new(self.params);

        for bed in 0..self.params.num_beds() {
            let bed_plan = self.get_bed_plan(bed);
            for bed_week in bed_plan.iter() {
                harvest_plan.add(bed_week.week, bed_week.crop, bed_week.harvestable_units);
            }
        } 

        harvest_plan
    }
}