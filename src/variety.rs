use std::convert::TryFrom;
use json::JsonValue;
use crate::common::*;
use crate::bed::{ BED_FLAG_POLYTUNNEL, BedFlags };
use crate::constant::{ SEASON_LENGTH, WeekRange, HarvestableUnits };

#[derive(Clone)]
pub struct Variety {
    pub name: String,
    // planting_schedule: [ bool; SEASON_LENGTH ],
    pub harvest_schedule: Vec<HarvestableUnits>,
    pub flags: BedFlags,
}

impl TryFrom<&JsonValue> for Variety {
    type Error = &'static str;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        let value_obj = as_object(&value)?;
        let name = as_string(&value_obj["name"])?;
        let flags = BedFlags::try_from(&value_obj["flags"])?;
        let harvest_schedule_arr = as_array(&value_obj["harvest_schedule"])?;
        let harvest_schedule = harvest_schedule_arr.iter().map(|j| as_int(j)).collect::<Result<Vec<_>, _>>()?;

        Ok(Variety {
            name: String::from(name),
            flags: flags,
            // planting_schedule: [ true; SEASON_LENGTH ],
            harvest_schedule: harvest_schedule,
        })
    }
}

#[cfg(test)]
#[test]
fn variety_from_json() {
    let js = json::parse(r#"
{
    "name": "tomato",
    "flags": [ "polytunnel" ],
    "harvest_schedule": [ 0, 1, 2, 3 ]
}"#).expect("test is wrong");
    let variety = Variety::try_from(&js).expect("failed to parse");
    assert_eq!(variety.name, "tomato");
    assert!(variety.flags.has_all(&BED_FLAG_POLYTUNNEL));
    assert_eq!(variety.harvest_schedule.len(), 4);
    assert_eq!(variety.harvest_schedule[2], 2);
}

impl Variety {
    pub fn get_longevity(&self) -> WeekRange {
        return self.harvest_schedule.len();
    }
}

// pub fn new() -> Box<Varieties> {
//     Box::new([
//         Variety{
//             name: "",
//             planting_schedule: [ false; SEASON_LENGTH ],
//             harvest_schedule: vec![ 0 ],
//         },

//         Variety{
//             name: "Spinach",
//             planting_schedule: [ false; SEASON_LENGTH ],
//             harvest_schedule: vec![ 0, 0, 0, 0, 0, 0, 2, 4, 6, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 4, 2 ],
//         },

//         Variety{
//             name: "Radish",
//             planting_schedule: [ false; SEASON_LENGTH ],
//             harvest_schedule: vec![ 0, 20 ],
//         },

//         Variety{
//             name: "Lettuce",
//             planting_schedule: [ false; SEASON_LENGTH ],
//             harvest_schedule: vec![ 0, 0, 0, 20 ],
//         },

//         Variety{
//             name: "Tomato",
//             planting_schedule: [ false; SEASON_LENGTH ],
//             harvest_schedule: vec![ 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5 ],
//         },

//         Variety{
//             name: "Carrot",
//             planting_schedule: [ false; SEASON_LENGTH ],
//             harvest_schedule: vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 12, 12, 12, 10, 10, 10 ],
//         },

//         Variety{
//             name: "Mescalin",
//             planting_schedule: [ false; SEASON_LENGTH ],
//             harvest_schedule: vec![ 0, 0, 10, 0, 8 ],
//         }
//     ])
// }