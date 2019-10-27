use crate::basket::Basket;
use std::fmt::Display;
use crate::constant::SEASON_LENGTH;
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
        for bed in 0..self.params.num_beds() {
            let plan = self.get_bed_plan(bed);
            plan.write_instructions(&mut result);
        }
        result
    }

    // This is the fitness function for the evolutionary algorithm
    pub fn score(&self) -> i32 {
        let mut score = -self.get_num_plantings();
        score -= self.get_total_basket_shortfall();

        score
    }

    fn get_total_basket_shortfall(&self) -> i32 {
        let expected_baskets = &self.params.baskets;
        let actual_baskets = self.get_baskets();

        let mut total_shortfall = 0;

        for week in 0..SEASON_LENGTH {
            let expected_basket = &expected_baskets[week];
            let actual_basket = &actual_baskets[week];
            let shortfall = expected_basket.total_shortfall(&actual_basket);
            total_shortfall += shortfall;
        }

        total_shortfall
    }

    pub fn get_bed_plan(&'a self, bed: usize) -> BedPlan<'a> {
        BedPlan::new(bed, &self.genes, self.params)
    }

    pub fn get_baskets(&self) -> Vec<Basket> {
        let mut baskets = std::iter::repeat(self.params).map(|p| Basket::new(p)).take(SEASON_LENGTH).collect::<Vec<_>>();
        for bed in 0..self.params.num_beds() {
            let bed_plan = self.get_bed_plan(bed);
            for bed_week in bed_plan.iter() {
                bed_week.write_basket(&mut baskets[bed_week.week]);
            }
        }
        baskets
    }

    fn get_num_plantings(&self) -> i32 {
        self.genes.iter().filter(|&x| *x != 0).count().try_into().unwrap()
    }
}

impl Display for Phenome<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        // bed utilization
        let mut utilization = 0.0;
        for bed in 0..self.params.num_beds() {
            utilization += self.get_bed_plan(bed).utilization();
        }
        utilization /= self.params.num_beds() as f32;


        // basket satisfaction
        let shortfall = self.get_total_basket_shortfall();

        writeln!(f, "Bed utilization: {:.0}%", utilization * 100.0)?;
        writeln!(f, "Basket shortfall: {} units", shortfall)?;

        writeln!(f, "Shortfall by week:")?;
        for i in 0..self.params.get_num_basket_categories() {
            write!(f, "{:<9}", self.params.get_basket_category_name(i))?;
        }
        writeln!(f, "")?;

        let actual_baskets = self.get_baskets();
        for w in 0..SEASON_LENGTH {
            for c in 0..self.params.get_num_basket_categories() {
                let shortfall = self.params.baskets[w].shortfall(&actual_baskets[w], c);
                write!(f, "{:<9}", shortfall)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}