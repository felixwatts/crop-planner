use crate::constant::SEASON_LENGTH;
use core::fmt::Display;
use crate::constant::VarietyId;
use crate::params::Params;

pub struct Formatter<'a> {
    params: &'a Params,
    planting_schedule: &'a Vec<VarietyId>
}

impl<'a> Formatter<'a> {
    pub fn new(params: &'a Params, planting_schedule: &'a Vec<VarietyId>) -> Formatter<'a> {
        Formatter{
            params: params,
            planting_schedule: planting_schedule
        }
    }
}

impl<'a> Display for Formatter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let evaluator = crate::evaluator::Evaluator::new(self.params, self.planting_schedule);

        let harvest_plan = evaluator.get_harvest_plan();

        write!(f, "{:>11}", "Week")?;
        for v in 1..self.params.varieties.len() {
            let variety = &self.params.varieties[v];
            let mut name = variety.name.clone();
            name.truncate(9);
            write!(f, "{:>11}", name)?;
        }
        writeln!(f)?;

        for week in 0..SEASON_LENGTH {
            write!(f, "{:>11}", week)?;
            for v in 1..self.params.varieties.len() {
                match self.params.varieties[v].is_harvestable_in_week(week) {
                    true => {
                        let harvestable_units = harvest_plan[v][week];
                        let satisfaction = std::cmp::min(harvestable_units, self.params.num_baskets) as f32 / self.params.num_baskets as f32;
                        write!(f, "{:>10.0}%", satisfaction * 100.0)?;
                    },
                    false => {
                        write!(f, "           ")?;
                    }

                }
            }
            writeln!(f)?;
        }

        writeln!(f, "Utilization: {:.0}%", evaluator.get_bed_utilization() * 100.0)?;
        writeln!(f, "Satisfaction: {:.0}%", evaluator.get_basket_satisfaction() * 100.0)?;
        writeln!(f, "Profit: {:.2}", evaluator.get_profit() as f32 / 100.0)
    }
}