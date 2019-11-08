use crate::constant::SEASON_LENGTH;
use crate::tasks::Tasks;
use crate::constant::VarietyId;
use crate::params::Params;
use crate::bed_plan::BedPlan;

pub struct Evaluator<'a> {
    params: &'a Params,
    planting_schedule: &'a Vec<VarietyId>
}

impl<'a> Evaluator<'a> {
    pub fn new(
        params: &'a Params, 
        planting_schedule: &'a Vec<VarietyId>) -> Self {
        Evaluator {
            params: params,
            planting_schedule: planting_schedule
        }
    }

    pub fn get_harvest_plan(&self) -> Vec<Vec<i32>> {
        let mut harvest_plan = vec![ vec![0i32; SEASON_LENGTH * 2]; self.params.varieties.len() ];

        for bed in 0..self.params.beds.len() {
            let bed_plan = self.get_bed_plan(bed);
            for bed_week in bed_plan.iter() {
                if bed_week.harvestable_units != 0 {
                    harvest_plan[bed_week.crop][bed_week.week] += bed_week.harvestable_units;
                }
            }
        }

        harvest_plan
    }

    pub fn get_basket_satisfaction(&self) -> f32 {

        let harvest_plan = self.get_harvest_plan();

        let mut potential = 0;
        let mut actual = 0;

        for variety in 0..self.params.varieties.len() {
             for week in 0..SEASON_LENGTH {
                if self.params.varieties[variety].is_harvestable_in_week(week) {
                    potential += 1;
                    if harvest_plan[variety][week] >= self.params.num_baskets {
                        actual += 1;
                    }
                }  
            }
        }

        actual as f32 / potential as f32
    }

    pub fn get_profit(&self) -> i32 {
        let mut result = 0;

        let harvest_plan = self.get_harvest_plan();
        for variety in 0..self.params.varieties.len() {
            for week in 0..SEASON_LENGTH*2 {
                let harvestable_units = harvest_plan[variety][week];
                let sellable_units = std::cmp::min(self.params.num_baskets, harvestable_units);
                let val = sellable_units * self.params.varieties[variety].value_per_unit;
                result += val; 
            }
        }

        result
    }

    pub fn get_bed_plan(&'a self, bed: usize) -> BedPlan<'a> {
        BedPlan::new(bed, &self.planting_schedule, self.params)
    }

    pub fn get_bed_utilization(&self) -> f32 {
        let mut utilization = 0.0;
        for bed in 0..self.params.num_beds() {
            utilization += self.get_bed_plan(bed).utilization();
        }
        utilization /= self.params.num_beds() as f32;

        utilization
    }

    pub fn get_tasks(&self) -> Tasks {
        let mut result = Tasks::new();
        for bed in 0..self.params.num_beds() {
            let plan = self.get_bed_plan(bed);
            plan.write_instructions(&mut result);
        }
        result
    }
}