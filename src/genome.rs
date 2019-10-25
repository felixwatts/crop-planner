use crate::constant::NUM_BOXES;
use crate::params::Params;
use crate::constant::{ SEASON_LENGTH, WeekId, VarietyId };
use crate::harvest_plan::HarvestPlan;
use std::convert::{TryInto, TryFrom};
use json::JsonValue;
use crate::common::*;

#[derive(Clone, Debug)]
pub struct Genome(Vec<VarietyId>);

pub struct BedStats<'a> {
    planting_schedule: &'a [VarietyId],
    params: &'a Params
}

impl BedStats<'_> {
    pub fn utilization(&self) -> f32 {
        let first_planting_opt = self.planting_schedule.iter().position(|&x| x != 0);
        if let Some(first_planting) = first_planting_opt {
            let mut occupied_weeks = 0.0;
            let mut occupied_counter = 0;
            let mut week = first_planting;
            loop {
                let planted_variety = self.planting_schedule[week];
                if planted_variety != 0 {
                    occupied_counter = self.params.varieties[planted_variety].get_longevity();
                }
                          
                if occupied_counter > 0 {
                    occupied_counter -= 1;
                    occupied_weeks += 1.0
                }

                week = (week + 1) % SEASON_LENGTH;

                if week == first_planting {
                    break;
                }
            }
            return occupied_weeks / SEASON_LENGTH as f32
        } else {
            return 0.0
        }        
    }
}

impl Genome {
    pub fn new(params: &Params) -> Self {
        Genome(vec![ 0; params.genome_size() ])
    }

    pub fn get_bed_stats<'a>(&'a self, bed: usize, params: &'a Params) -> BedStats<'a> {
        let slice = &self.0[(bed*SEASON_LENGTH)..(bed*SEASON_LENGTH+SEASON_LENGTH)];
        BedStats{
            planting_schedule: slice,
            params: params
        }
    }

    pub fn get_variety(&self, bed: usize, week: usize) -> VarietyId {
        self.0[bed*SEASON_LENGTH+week]
    }

    pub fn randomize(&mut self, rand: &mut crate::rand::Rand) {
        for gene in 0..self.0.len() {
            self.randomize_gene(gene, rand);
        }
    }

    pub fn cross(mother: &Self, father: &Self, child: &mut Self, rand: &mut crate::rand::Rand) {
        for gene in 0..mother.0.len() {
            let variety = match rand.random_parent() {
                true => { mother.0[gene] }
                false => { father.0[gene] }
            };
            child.0[gene] = variety;
        }
    }

    pub fn mutate(&mut self, rand: &mut crate::rand::Rand) {
        let gene = rand.random_gene();
        self.randomize_gene(gene, rand);
    }

    pub fn to_harvest_plan(&self, params: &Params) -> HarvestPlan {
        // build the harvest plan, which tells us how many units of each variety
        // are harvestable in each week
        let mut harvest_plan = std::iter::repeat(vec![ 0; params.varieties.len() ])
            .take(SEASON_LENGTH)
            .collect::<Vec<_>>();
        for bed in 0..params.num_beds() {
            for planting_week in 0..SEASON_LENGTH {
                let variety_id = self.get_variety(bed, planting_week);
                let variety = &params.varieties[variety_id];

                for growth_week in 1..variety.get_longevity() {
                    let harvest_week = (planting_week+growth_week) % SEASON_LENGTH;


                    if self.get_variety(bed, harvest_week) != 0 {
                        break;
                    }

                    let harvest_units = variety.harvest_schedule[growth_week];
                    harvest_plan[harvest_week][variety_id] += harvest_units;
                }
            }
        }

        return harvest_plan;
    }

    pub fn score(&self, params: &Params) -> i32 {
        // minimize plantings
        let mut score = -self.count_plantings();

        let harvest_plan = self.to_harvest_plan(params);

        // aim in each week to have the harvestable units of each crop equal to the number of boxes
        for week in 0..SEASON_LENGTH {
            let harvest = &harvest_plan[week];

            for variety_id in 0..params.varieties.len() {
                let harvestable_units = harvest[variety_id];

                score -= (harvestable_units - NUM_BOXES).abs();
            }
        }

        score
    }

    fn randomize_gene(&mut self, gene: usize, rand: &mut crate::rand::Rand) {
        let week = gene % SEASON_LENGTH;
        self.0[gene] = rand.random_variety(week);
    }

    fn count_plantings(&self) -> i32 {
        self.0.iter().filter(|&x| *x != 0).count().try_into().unwrap()
    }
}

impl std::convert::Into<json::JsonValue> for Genome {
    fn into(self) -> json::JsonValue {
        self.0.clone().into()
    }
}

impl TryFrom<&JsonValue> for Genome {
    type Error = &'static str;
    
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        let value_arr = as_array(value)?;
        let genes = value_arr.iter().map(|j| as_usize(j)).collect::<Result<Vec<_>, _>>()?;
        Ok(Genome(genes))
    }
}

// pub fn print_solution(sol: &Genome, params: &Params) {
//     for bed in 0..params.beds.len() {
//         for crop in 0..SEASON_LENGTH {
//             let gene_id = bed * SEASON_LENGTH + crop;
//             let variety_id = sol[gene_id];
//             let variety = &params.varieties[variety_id];
//             print!("[{}] ", variety.name);
//         }
//         println!("");
//     }
// }