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

    // Returns a number between 0 and 1 representing the extent to which each variety that could
    // possibly be harvested in each week can be harvested with enough quantity to fill the market
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

    // returns the profit in value units realized by this planting schedule
    // over one season. Note that this captures the value harvested between jan1 and dec 31
    // and doesn't capture the value of crops planted in that time but not harvested
    // until next season
    pub fn get_profit(&self) -> i32 {
        self._get_profit(SEASON_LENGTH)
    }

    // The fitness function used by the evolutionary algorithm. Captures the value of
    // all crops harvested _or planted_ between jan1 and dec 31. This encourages plans
    // that set up good overwintered crops for next year.
    pub fn get_fitness(&self) -> i32 {
        self._get_profit(SEASON_LENGTH*2)
    }

    fn _get_profit(&self, season_length: usize) -> i32 {
        // TODO model cost of production better
        let cost: i32 = self.planting_schedule.iter().map(|x| match x { 0 => 0, _ => 1}).sum();
        let mut profit = -cost;

        let harvest_plan = self.get_harvest_plan();
        for variety in 0..self.params.varieties.len() {
            for week in 0..season_length {
                let harvestable_units = harvest_plan[variety][week];
                let sellable_units = std::cmp::min(self.params.num_baskets, harvestable_units);
                let val = sellable_units * self.params.varieties[variety].value_per_unit;
                profit += val; 
            }
        }

        profit
    }

    pub fn get_bed_utilization(&self) -> f32 {
        let mut utilization = 0.0;
        for bed in 0..self.params.beds.len() {
            utilization += self.get_bed_plan(bed).utilization();
        }
        utilization /= self.params.beds.len() as f32;

        utilization
    }

    pub fn get_tasks(&self) -> Tasks {
        let mut result = Tasks::new();
        for bed in 0..self.params.beds.len() {
            let plan = self.get_bed_plan(bed);
            plan.write_instructions(&mut result);
        }
        result
    }

    fn get_bed_plan(&'a self, bed: usize) -> BedPlan<'a> {
        BedPlan::new(bed, &self.planting_schedule, self.params)
    }
}