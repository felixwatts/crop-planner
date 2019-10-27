use crate::basket::Basket;
use crate::tasks::Tasks;
use crate::variety::Variety;
use crate::constant::WeekId;
use crate::constant::{VarietyId, SEASON_LENGTH};
use crate::params::Params;
use crate::bed::Bed;

// Represents part of a crop plan relating to a single bed
// Provides methods to extract instructions and statistics and print
pub struct BedPlan<'a> {
    planting_schedule: &'a [VarietyId],
    params: &'a Params,
    def: &'a Bed
}

impl BedPlan<'_> {
    pub fn new<'a>(bed: usize, genes: &'a Vec<VarietyId>, params: &'a Params) -> BedPlan<'a> {
        let planting_schedule = &genes[(bed*SEASON_LENGTH)..(bed*SEASON_LENGTH+SEASON_LENGTH)];
        BedPlan{
            planting_schedule: planting_schedule,
            params: params,
            def: &params.beds[bed]
        }
    }

    pub fn iter<'a>(&'a self) -> BedPlanIterator<'a> {
        BedPlanIterator::new(self.planting_schedule, self.params)
    }

    pub fn write_instructions(&self, tasks: &mut Tasks) {
        for bed_week in self.iter() {
            self.write_planting_instructions(&bed_week, tasks);
            self.write_harvesting_instructions(&bed_week, tasks);
        }
    }

    fn write_planting_instructions(&self, bed_week: &BedWeek, tasks: &mut Tasks) {
        match bed_week.get_planted_variety() {
            Some(planted_variety) => {
                for week_offset in -52..52 {
                    let instruction_template_opt = &planted_variety.instructions.get(&week_offset.to_string());
                    if let Some(instruction_template) = instruction_template_opt {
                        let instruction = instruction_template
                            .replace("<variety>", &planted_variety.name)
                            .replace("<label>", &format!("{}-{}", self.def.name, bed_week.week))
                            .replace("<bed>", &self.def.name);
                        let mut week = week_offset + (bed_week.week as i32);
                        while week < 0 {
                            week += SEASON_LENGTH as i32
                        }
                        tasks.add(week as usize, &instruction);
                    };
                }
            },
            None => ()
        }
    }

    fn write_harvesting_instructions(&self, bed_week: &BedWeek, tasks: &mut Tasks) {

        if bed_week.harvestable_units == 0 { return }

        let harvested_variety = bed_week.get_growing_variety().unwrap();

        if let Some(harvest_instruction_template) = harvested_variety.instructions.get("harvest") {
            let harvest_instruction = harvest_instruction_template
                .replace("<variety>", &harvested_variety.name)
                .replace("<label>", &format!("{}-{}", self.def.name, bed_week.week))
                .replace("<units>", &bed_week.harvestable_units.to_string())
                .replace("<bed>", &self.def.name);
            tasks.add(bed_week.week, &harvest_instruction)
        };
    }

    pub fn utilization(&self) -> f32 {

        let mut occupied_weeks = 0.0;
        for bed_week in self.iter() {
            match bed_week.get_growing_variety() {
                Some(_) => occupied_weeks += 1.0,
                None => ()
            }
        }
        occupied_weeks / SEASON_LENGTH as f32
    }

    fn get_variety(&self, week: usize) -> VarietyId {
        self.planting_schedule[week]
    }
}

impl std::fmt::Display for BedPlan<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "Bed {}\n", self.def.name)?;
        writeln!(f, "{:<9}{:<9}", "Week", "Variety")?;
        for week in 0..SEASON_LENGTH {
            let variety = self.get_variety(week);
            if variety != 0 {
                let variety_name = &self.params.varieties[variety].name;
                println!("{:<9}{:<9}", week, variety_name);
            }
        }
        writeln!(f)?;
        writeln!(f, "Utilization: {:.0}%", self.utilization() * 100.0)?;
        Ok(())
    }
}

pub struct BedWeek<'a> {
    pub week: WeekId,
    pub crop: VarietyId,
    pub crop_age: usize,
    pub harvestable_units: i32,
    params: &'a Params
}

impl<'a> BedWeek<'a> {
    fn get_growing_variety(&self) -> Option<&'a Variety> {
        if self.crop != 0 { Some(&self.params.varieties[self.crop]) } else { None }
    }
    fn get_planted_variety(&self) -> Option<&'a Variety> {
        if self.crop_age == 0 { Some(&self.params.varieties[self.crop]) } else { None }
    }
    pub fn write_basket(&self, basket: &mut Basket) {
        if self.harvestable_units != 0 {
            basket.add(self.get_growing_variety().unwrap().basket_category, self.harvestable_units);
        }
    }
}

pub struct BedPlanIterator<'a> {
    planting_schedule: &'a[usize],
    first_planting: Option<usize>,
    week: usize,
    planted_variety: VarietyId,
    planted_age: usize,
    params: &'a Params
}

impl<'a> BedPlanIterator<'a> {
    pub fn new(planting_schedule: &'a[usize], params: &'a Params) -> BedPlanIterator<'a> {
        let first_planting = planting_schedule.iter().position(|&x| x != 0);
        BedPlanIterator {
            planting_schedule: planting_schedule,
            first_planting: first_planting,
            week: first_planting.or(Some(0)).unwrap(),
            planted_variety: 0,
            planted_age: 0,
            params: params
        }
    }
}

impl<'a> Iterator for BedPlanIterator<'a> {
    type Item = BedWeek<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.first_planting {
            None => None,
            Some(fp) => {
                if self.planting_schedule[self.week] != 0 {
                    self.planted_variety = self.planting_schedule[self.week];
                    self.planted_age = 0;
                }

                if self.planted_variety != 0 && self.planted_age >= self.params.varieties[self.planted_variety].get_longevity() {
                    self.planted_variety = 0;
                    self.planted_age = 0;
                }

                let mut result = BedWeek{
                    week: self.week,
                    crop: self.planted_variety,
                    crop_age: self.planted_age,
                    harvestable_units: 0,
                    params: self.params
                };

                if self.planted_variety != 0 {
                    result.harvestable_units = self
                        .params
                        .varieties[self.planted_variety]
                        .harvest_schedule[self.planted_age];
                }

                if self.planted_variety != 0 {

                }

                self.week = (self.week + 1) % SEASON_LENGTH;
                self.planted_age += 1;

                if self.week == fp { None } else { Some(result) }
            }
        }
    }
}

