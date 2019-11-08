use crate::tasks::Tasks;
use crate::constant::VarietyId;
use crate::params::Params;
use crate::bed_plan::BedPlan;
use std::convert::TryInto;

pub struct Evaluator<'a> {
    params: &'a Params,
    planting_schedule: &'a Vec<VarietyId>
}

impl<'a> Evaluator<'a> {
    pub fn new(params: &'a Params, planting_schedule: &'a Vec<VarietyId>) -> Self {
        Evaluator {
            params: params,
            planting_schedule: planting_schedule
        }
    }

    pub fn get_score(&self) -> i32 {
        0
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

    fn get_num_plantings(&self) -> i32 {
        self.planting_schedule.iter().filter(|&x| *x != 0).count().try_into().unwrap()
    }
}