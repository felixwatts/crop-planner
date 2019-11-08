use crate::bed::Bed;
use crate::constant::VarietyId;
use crate::constant::NUM_BASKETS;
use crate::variety::Variety;
use crate::params::Params;
use crate::constant::SEASON_LENGTH;

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct CfVariety<'a> {
    id: usize,
    variety: &'a Variety
}

impl<'a> CfVariety<'_> {
    fn new(params: &'a Params, id: usize) -> CfVariety<'a> {
        CfVariety {
            id: id,
            variety: &params.varieties[id]
        }
    }

    fn are_requirements_met(&self, bed: &Bed) -> bool {
        self.variety.are_requirements_met(bed)
    }

    fn same(left: &CfVariety<'_>, right: &CfVariety<'_>) -> bool {
        left.id == right.id
    }

    fn get_value_score(&self) -> u32 {
        0
    }

    fn get_longevity(&self) -> usize {
        self.variety.get_longevity()
    }

    fn get_num_weeks_to_maturity(&self) -> Option<usize> {
        self.variety.harvest_schedule.iter().position(|&h| h != 0)
    }

    fn get_harvestable_units(&self, week: usize) -> i32 {
        self.variety.harvest_schedule[week]
    }

    fn get_harvestable_weeks(&self) -> [bool; SEASON_LENGTH] {
        let mut result = [false; SEASON_LENGTH];
        for start_week in 0..SEASON_LENGTH {
            if self.variety.planting_schedule[start_week] {
                for growth_week in 0..self.variety.harvest_schedule.len() {
                    if self.variety.harvest_schedule[growth_week] != 0 {
                        result[(start_week+growth_week)%SEASON_LENGTH] = true
                    }
                }
            }
            
        }
        result
    }
}

struct CfBed<'a> {
    id: usize,
    params: &'a Params,
    schedule: [(Option<&'a CfVariety<'a>>, Option<&'a CfVariety<'a>>, i32); SEASON_LENGTH]
}

impl<'a> CfBed<'a> {
    fn new(params: &'a Params, id: usize) -> CfBed<'a> {
        CfBed {
            id: id,
            params: params,
            schedule: [(None, None, 0); SEASON_LENGTH]
        }
    }

    pub fn try_plant(&mut self, variety: &'a CfVariety<'a>, week: usize) -> bool {
        let bed = &self.params.beds[self.id];
        if !variety.are_requirements_met(bed) { return false; }

        if !self.is_range_free(week, variety.get_longevity()) { return false; }

        self.plant(variety, week);

        true
    }

    pub fn get_harvestable_units(&self, variety: &'a CfVariety<'a>, week: usize) -> i32 {
        match self.schedule[week].1 {
            Some(harvest_variety) => {
                if CfVariety::same(variety, harvest_variety) {
                    self.schedule[week].2 
                } else {
                    0
                }
            },
            None => 0
        }
    }

    pub fn write_genes(&self, genes: &mut Vec<VarietyId>) {
        let start = self.id*SEASON_LENGTH;
        let end = start + SEASON_LENGTH;
        let sl = &mut genes[start..end];
        let x = (0..SEASON_LENGTH)
            .map(|w| match self.schedule[w].0 { Some(v) => v.id, None => 0 })
            .collect::<Vec<_>>();
        // TODO tidy up
        for y in 0..x.len() {
            sl[y] = x[y];
        }
    }

    fn is_range_free(&self, start: usize, len: usize) -> bool {
        (start..start+len).map(|w| self.schedule[w%SEASON_LENGTH].1).all(|s| match s { Some(_) => false, None => true })
    }

    fn plant(&mut self, variety: &'a CfVariety<'a>, week: usize) {
        self.schedule[week%SEASON_LENGTH].0 = Some(variety);
        for growth_week in 0..variety.get_longevity() {
            self.schedule[(week + growth_week)%SEASON_LENGTH].1 = Some(variety);
            self.schedule[(week + growth_week)%SEASON_LENGTH].2 = variety.get_harvestable_units(growth_week);
        }
    }
}

pub struct CfPlanner<'a> {
    params: &'a Params,
    beds: Vec<CfBed<'a>>,
    varieties: Vec<CfVariety<'a>>
}

impl<'a> CfPlanner<'a> {
    pub fn new(params: &'a Params) -> CfPlanner<'a> {        
        let mut planner = CfPlanner {
            params: params,
            beds: (0..params.beds.len())
                .map(|b| CfBed::new(params, b))
                .collect::<Vec<_>>(),
            varieties: (0..params.varieties.len())
                .map(|v| CfVariety::new(params, v))
                .collect::<Vec<_>>()
        };

        planner.varieties.sort_by_key(|v| v.get_value_score());

        planner
    }

    pub fn plan(&'a mut self) -> Vec<usize> {
        for variety in self.varieties.iter() {
            let harvestable_weeks = variety.get_harvestable_weeks();
            for harvest_week in 0..harvestable_weeks.len() {
                if !harvestable_weeks[harvest_week] { 
                    continue; 
                }

                let planting_week = if harvest_week >= variety.get_num_weeks_to_maturity().unwrap() {
                    harvest_week - variety.get_num_weeks_to_maturity().unwrap()
                } else {
                    (harvest_week + SEASON_LENGTH) - variety.get_num_weeks_to_maturity().unwrap()
                };

                for b in 0..self.beds.len() {
                    if self.get_harvestable_units(variety, harvest_week) >= NUM_BASKETS {
                        break;
                    }

                    self.beds[b].try_plant(&variety, planting_week);
                }
            }
        }

        let mut result = vec![0; self.params.genome_size()];

        for bed in self.beds.iter() {
            bed.write_genes(&mut result);
        }

        result
    }

    fn get_harvestable_units(&self, variety: &'a CfVariety<'a>, week: usize) -> i32 {
        self
            .beds
            .iter()
            .fold(0, |c, b| c + b.get_harvestable_units(variety, week))
    }
 }