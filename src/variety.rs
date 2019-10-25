use std::convert::TryFrom;
use json::JsonValue;
use crate::common::*;
use crate::bed::{BedFlags, BED_FLAG_POLYTUNNEL};
use crate::constant::{ SEASON_LENGTH, WeekRange, HarvestableUnits };

#[derive(Clone)]
pub struct Variety {
    pub name: String,
    pub planting_schedule: [ bool; SEASON_LENGTH ],
    pub harvest_schedule: Vec<HarvestableUnits>,
    pub flags: BedFlags,
}

fn try_parse_planting_schedule(input: &str) -> Result<[ bool; SEASON_LENGTH ], &'static str> {
    let mut result = [ false; SEASON_LENGTH ];
    let parts = input.split(',');
    let months = [ "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec" ];
    for part in parts {
        let index = months.iter().position(|&m| m == part);
        match index {
            Some(i) => {
                for n in 0..4 {
                    result[i*4+n] = true;
                }
            },
            None => return Err("Failed to parse harvest schedule")
        }
    }
    Ok(result)
}

impl TryFrom<&JsonValue> for Variety {
    type Error = &'static str;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        let value_obj = as_object(&value)?;
        let name = as_string(&value_obj["name"])?;
        let flags = BedFlags::try_from(&value_obj["flags"])?;
        let harvest_schedule_arr = as_array(&value_obj["harvest_schedule"])?;
        let harvest_schedule = harvest_schedule_arr.iter().map(|j| as_int(j)).collect::<Result<Vec<_>, _>>()?;
        let planting_schedule_str = as_string(&value_obj["planting_schedule"])?;
        let planting_schedule = try_parse_planting_schedule(&planting_schedule_str)?;

        Ok(Variety {
            name: String::from(name),
            flags: flags,
            planting_schedule: planting_schedule,
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
    "harvest_schedule": [ 0, 1, 2, 3 ],
    "planting_schedule": "apr,may"
}"#).expect("test is wrong");
    let variety = Variety::try_from(&js).expect("failed to parse");
    assert_eq!(variety.name, "tomato");
    assert!(variety.flags.has_all(&BED_FLAG_POLYTUNNEL));
    assert_eq!(variety.harvest_schedule.len(), 4);
    assert_eq!(variety.harvest_schedule[2], 2);
    assert_eq!(variety.planting_schedule[11], false);
    assert_eq!(variety.planting_schedule[12], true);
}

impl Variety {
    pub fn get_longevity(&self) -> WeekRange {
        return self.harvest_schedule.len();
    }
}