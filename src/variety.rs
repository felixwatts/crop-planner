use crate::bed::Bed;
use json::JsonValue;
use crate::common::*;
use crate::constant::{ SEASON_LENGTH, WeekRange, HarvestableUnits };
use std::collections::HashMap;
use std::error::Error;
use simple_error::*;
use regex::Regex;
use crate::params::Params;

// Represents a variety of crop
// Loaded from params.json and used as part of the input parameters to the plan generating algorithm
#[derive(Clone)]
pub struct Variety {
    pub name: String,
    pub planting_schedule: [ bool; SEASON_LENGTH ],
    pub harvest_schedule: Vec<HarvestableUnits>,
    pub requirements: Vec<String>,
    pub instructions: HashMap<String, String>,
    pub basket_category: usize
}

impl Variety {
    pub fn are_requirements_met(&self, bed: &Bed) -> bool {
        self.requirements.iter().all(|r| bed.properties.contains(r))
    }

    pub fn try_parse(value: &JsonValue, params: &mut Params) -> Result<Self, Box<dyn Error>> {
        let value_obj = as_object(&value)?;
        let name = as_string(&value_obj["name"])?;

        let basket_category_name = as_string(&value_obj["basket_category"])?;
        let basket_category = params.get_basket_category_id(&basket_category_name);

        let requirements = match &value_obj["requirements"] {
            JsonValue::Array(arr) => arr.iter().map(|p| as_string(p)).collect::<Result<Vec<_>,_>>(),
            JsonValue::Null => Ok(vec![]),
            _ => bail!("Invalid requirements")
        }?;

        let planting_schedule_str = as_string(&value_obj["planting_schedule"])?;
        let mut planting_schedule = [false; SEASON_LENGTH];

        lazy_static! {
            static ref REGEX_PLANTING_SCHEDULE: Regex = Regex::new("([0-9]+)(-([0-9]+))?").unwrap();
        }
        for cap in REGEX_PLANTING_SCHEDULE.captures_iter(&planting_schedule_str) {
            match cap.get(3) {
                None => {
                    let week = &cap[1].parse::<usize>()?;
                    planting_schedule[*week] = true;
                },
                Some(_) => {
                    let start_week = &cap[1].parse::<usize>()?;
                    let end_week = &cap[3].parse::<usize>()?;
                    if start_week >= end_week {
                        bail!("Invalid planting schedule")
                    }
                    for week in *start_week..=*end_week {
                        planting_schedule[week] = true;
                    }
                }
            }

            // println!("{} {} {} {}", &cap[0], &cap[1], &cap[2], &cap[3]);
        }

        let harvest_schedule_arr = as_array(&value_obj["harvest_schedule"])?;
        let harvest_schedule = harvest_schedule_arr.iter().map(|j| as_int(j)).collect::<Result<Vec<_>, _>>()?;
        let instructions = try_parse_instructions(&value_obj["instructions"])?;

        Ok(Variety {
            name: String::from(name),
            requirements: requirements,
            planting_schedule: planting_schedule,
            harvest_schedule: harvest_schedule,
            instructions: instructions,
            basket_category: basket_category
        })
    }
}

fn try_parse_instructions(input: &JsonValue) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let input_obj = as_object(input)?;

    let mut result = HashMap::new();

    for item in input_obj.iter() {
        let key = std::string::String::from(item.0);
        let val = as_string(item.1)?;
        result.insert(key, std::string::String::from(val));
    }

    Ok(result)
}

#[cfg(test)]
#[test]
fn variety_from_json() {
    let mut params = Params{
        beds: vec![],
        varieties: vec![],
        baskets: vec![],
        basket_category_names: vec![]
    };
    let js = json::parse(r#"
{
    "name": "tomato",
    "requirements": [ "polytunnel" ],
    "basket_category": "tomato",
    "harvest_schedule": [ 0, 1, 2, 3 ],
    "planting_schedule": "4-8,20-24,40",
    "instructions": {
        "-6": "Seed <variety> into a 64 tray and label it <label>",
        "-4": "Transplant <variety> from tray <label> into 20cm pots and label them <label>",
        "0": "Transplant <variety> from pots labelled <label> into bed <bed>"
    }
}"#).expect("test is wrong");
    let variety = Variety::try_parse(&js, &mut params).expect("failed to parse");
    assert_eq!(variety.name, "tomato");
    assert!(variety.requirements.contains(&String::from("polytunnel")));
    assert!(!variety.requirements.contains(&String::from("magic")));
    assert_eq!(variety.harvest_schedule.len(), 4);
    assert_eq!(variety.harvest_schedule[2], 2);
    assert_eq!(variety.planting_schedule[3], false);
    assert_eq!(variety.planting_schedule[4], true);
    assert_eq!(variety.planting_schedule[8], true);
    assert_eq!(variety.planting_schedule[9], false);
    assert_eq!(variety.planting_schedule[19], false);
    assert_eq!(variety.planting_schedule[20], true);
    assert_eq!(variety.planting_schedule[24], true);
    assert_eq!(variety.planting_schedule[25], false);
    assert_eq!(variety.planting_schedule[39], false);
    assert_eq!(variety.planting_schedule[40], true);
    assert_eq!(variety.planting_schedule[41], false);
    assert_eq!(variety.instructions["-6"], "Seed <variety> into a 64 tray and label it <label>");
    assert_eq!(variety.instructions["0"], "Transplant <variety> from pots labelled <label> into bed <bed>");
    assert_eq!(params.get_basket_category_name(variety.basket_category), "tomato");
}

impl Variety {
    // Get how long the crop lasts from planting out to last harvest
    pub fn get_longevity(&self) -> WeekRange {
        return self.harvest_schedule.len();
    }
}