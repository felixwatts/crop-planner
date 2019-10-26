use crate::constant::NUM_BOXES;
use crate::constant::SEASON_LENGTH;
use crate::harvest_plan::HarvestPlan;
use crate::bed_plan::BedPlan;
use crate::instructions::Instructions;
use crate::params::Params;
use std::convert::TryInto;

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

    pub fn get_instructions(&self) -> Instructions {
        let mut result = Instructions::new();
        for bed in 0..self.params.beds.len() {
            let plan = self.get_bed_plan(bed);
            plan.write_instructions(&mut result);
        }
        result
    }

    pub fn score(&self) -> i32 {
        // minimize plantings
        let num_plantings: i32 = self.genes.iter().filter(|&x| *x != 0).count().try_into().unwrap();
        let mut score = -num_plantings;

        let harvest_plan = self.get_harvest_plan();

        // aim in each week to have the harvestable units of each crop equal to the number of boxes
        for week in 0..SEASON_LENGTH {
            let harvest = &harvest_plan[week];

            for variety_id in 0..self.params.varieties.len() {
                let harvestable_units = harvest[variety_id];

                score -= (harvestable_units - NUM_BOXES).abs();
            }
        }

        score
    }

    pub fn get_bed_plan(&'a self, bed: usize) -> BedPlan<'a> {
        BedPlan::new(bed, &self.genes, self.params)
    }

    pub fn get_harvest_plan(&self) -> HarvestPlan {
        // build the harvest plan, which tells us how many units of each variety
        // are harvestable in each week

        let mut harvest_plan = std::iter::repeat(vec![ 0; self.params.varieties.len() ])
            .take(SEASON_LENGTH)
            .collect::<Vec<_>>();

        for b in 0..self.params.num_beds() {
            let bed = self.get_bed_plan(b);
            for w in bed.iter() {
                if w.harvest_units != 0 {
                    harvest_plan[w.week][w.variety] += w.harvest_units;
                }
            }
        } 

        harvest_plan
    }
}