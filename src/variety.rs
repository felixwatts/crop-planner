use crate::constant::{ NUM_VARIETIES, SEASON_LENGTH, WeekRange, HarvestableUnits };

pub struct Variety {
    pub name: &'static str,
    planting_schedule: [ bool; SEASON_LENGTH ],
    pub harvest_schedule: Vec<HarvestableUnits>,
}

pub type Varieties = [Variety; NUM_VARIETIES];

impl Variety {
    pub fn get_longevity(&self) -> WeekRange {
        return self.harvest_schedule.len();
    }
}

pub fn new() -> Box<Varieties> {
    Box::new([
        Variety{
            name: "",
            planting_schedule: [ false; SEASON_LENGTH ],
            harvest_schedule: vec![ 0 ],
        },

        Variety{
            name: "Spinach",
            planting_schedule: [ false; SEASON_LENGTH ],
            harvest_schedule: vec![ 0, 0, 0, 0, 0, 0, 2, 4, 6, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 4, 2 ],
        },

        Variety{
            name: "Radish",
            planting_schedule: [ false; SEASON_LENGTH ],
            harvest_schedule: vec![ 0, 20 ],
        },

        Variety{
            name: "Lettuce",
            planting_schedule: [ false; SEASON_LENGTH ],
            harvest_schedule: vec![ 0, 0, 0, 20 ],
        },

        Variety{
            name: "Tomato",
            planting_schedule: [ false; SEASON_LENGTH ],
            harvest_schedule: vec![ 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5 ],
        },

        Variety{
            name: "Carrot",
            planting_schedule: [ false; SEASON_LENGTH ],
            harvest_schedule: vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 12, 12, 12, 10, 10, 10 ],
        },

        Variety{
            name: "Mescalin",
            planting_schedule: [ false; SEASON_LENGTH ],
            harvest_schedule: vec![ 0, 0, 10, 0, 8 ],
        }
    ])
}