use crate::constant::WeekId;
use crate::instructions::Instructions;
use crate::constant::{VarietyId, SEASON_LENGTH};
use crate::params::Params;
use crate::bed::Bed;

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

    pub fn write_instructions(&self, ins: &mut Instructions) {
        for item in self.iter() {
            let instructions = &self.params.varieties[item.variety].instructions;

            if item.age == 0 {
                // planted this week
                for w in -52..52 {
                    let instruction = instructions.get(&w.to_string());
                    match instruction {
                        Some(i) => {
                            let mut i2 = i.replace("<variety>", &self.params.varieties[item.variety].name);
                            i2 = i2.replace("<label>", &format!("{}-{}", self.def.name, item.week));
                            i2 = i2.replace("<bed>", &self.def.name);
                            let mut week = w + (item.week as i32);
                            while week < 0 {
                                week += SEASON_LENGTH as i32
                            }
                            ins.add(week as usize, &i2)
                        },
                        None => ()
                    };
                }
            };

            if item.harvest_units != 0 {
                // harvested this week
                let instruction = instructions.get("harvest");
                match instruction {
                    Some(i) => {
                        let mut i2 = i.replace("<variety>", &self.params.varieties[item.variety].name);
                        i2 = i2.replace("<label>", &format!("{}-{}", self.def.name, item.week));
                        i2 = i2.replace("<bed>", &self.def.name);
                        i2 = i2.replace("<units>", &item.harvest_units.to_string());
                        ins.add(item.week, &i2)
                    },
                    None => ()
                };
            };
        }
    }

    pub fn utilization(&self) -> f32 {

        let mut occupied_weeks = 0.0;
        for item in self.iter() {
            if item.variety != 0 {
                occupied_weeks += 1.0;
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

#[derive(Debug)]
pub struct BedWeek {
    pub week: WeekId,
    pub variety: VarietyId,
    pub age: usize,
    pub harvest_units: i32
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

impl Iterator for BedPlanIterator<'_> {
    type Item = BedWeek;

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
                    variety: self.planted_variety,
                    age: self.planted_age,
                    harvest_units: 0
                };

                if self.planted_variety != 0 {
                    result.harvest_units = self
                        .params
                        .varieties[self.planted_variety]
                        .harvest_schedule[self.planted_age]
                }

                self.week = (self.week + 1) % SEASON_LENGTH;
                self.planted_age += 1;

                if self.week == fp { None } else { Some(result) }
            }
        }
    }
}

