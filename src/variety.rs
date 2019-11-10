use crate::bed::Bed;
use json::JsonValue;
use crate::common::*;
use crate::constant::{ SEASON_LENGTH, WeekRange, HarvestableUnits };
use std::collections::HashMap;
use std::error::Error;
use simple_error::*;
use regex::Regex;

// Represents a variety of crop
// Loaded from params.json and used as part of the input parameters to the plan generating algorithm
#[derive(Clone)]
pub struct Variety {
    pub name: String,
    pub planting_schedule: [ bool; SEASON_LENGTH ],
    pub harvest_schedule: Vec<HarvestableUnits>,
    harvestable_by_week: Vec<bool>,
    pub requirements: Vec<String>,
    pub instructions: HashMap<String, String>,
    pub value_per_unit: i32
}

impl Variety {
    pub fn are_requirements_met(&self, bed: &Bed) -> bool {
        self.requirements.iter().all(|r| bed.properties.contains(r))
    }

    pub fn try_parse(value: &JsonValue) -> Result<Self, Box<dyn Error>> {
        let value_obj = as_object(&value)?;
        let name = as_string(&value_obj["name"])?;

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
        }

        let mut harvest_schedule = vec![];
        let harvest_schedule_str = as_string(&value_obj["harvest_schedule"])?;
        lazy_static! {
            static ref REGEX_HARVEST_SCHEDULE: Regex = Regex::new("([0-9]+)(:([0-9]+))?").unwrap();
        }
        for cap in REGEX_HARVEST_SCHEDULE.captures_iter(&harvest_schedule_str) {
            match cap.get(3) {
                None => {
                    let harvestable_units = &cap[1].parse::<i32>()?;
                    harvest_schedule.push(*harvestable_units);
                },
                Some(_) => {
                    let harvestable_units = &cap[1].parse::<i32>()?;
                    let run_length = &cap[3].parse::<i32>()?;
                    for _ in 0..*run_length {
                        harvest_schedule.push(*harvestable_units);
                    }
                }
            }
        }

        let instructions = try_parse_instructions(&value_obj["instructions"])?;

        let value_per_unit = as_int(&value_obj["value_per_unit"])?;

        let mut harvestable_by_week = vec![false; SEASON_LENGTH];
        for planting_week in 0..SEASON_LENGTH {
            if planting_schedule[planting_week] {
                for growth_week in 0..harvest_schedule.len() {
                    if harvest_schedule[growth_week] != 0 {
                        let harvest_week = (planting_week+growth_week) % SEASON_LENGTH;
                        harvestable_by_week[harvest_week] = true;
                    }
                }
            }
        }

        Ok(Variety {
            name: String::from(name),
            requirements: requirements,
            planting_schedule: planting_schedule,
            harvest_schedule: harvest_schedule,
            instructions: instructions,
            value_per_unit: value_per_unit,
            harvestable_by_week: harvestable_by_week
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
    let js = json::parse(r#"
{
    "name": "tomato",
    "requirements": [ "polytunnel" ],
    "harvest_schedule": "0:3,4,5:2",
    "planting_schedule": "4-8,20-24,40,50",
    "instructions": {
        "-6": "Seed <variety> into a 64 tray and label it <label>",
        "-4": "Transplant <variety> from tray <label> into 20cm pots and label them <label>",
        "0": "Transplant <variety> from pots labelled <label> into bed <bed>"
    },
    "value_per_unit": 100
}"#).expect("test is wrong");
    let variety = Variety::try_parse(&js).expect("failed to parse");
    assert_eq!(variety.name, "tomato");
    assert!(variety.requirements.contains(&String::from("polytunnel")));
    assert!(!variety.requirements.contains(&String::from("magic")));
    assert_eq!(variety.harvest_schedule, vec![0,0,0,4,5,5]);
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
    assert_eq!(variety.value_per_unit, 100);
    assert_eq!(variety.harvestable_by_week[0], false);
    assert_eq!(variety.harvestable_by_week[1], true);
}

impl Variety {

    pub fn empty() -> Self {
        Variety{
            name: "".to_string(),
            harvest_schedule: vec![],
            planting_schedule: [false;SEASON_LENGTH],
            instructions: std::collections::HashMap::new(),
            requirements: vec![],
            value_per_unit: 100,
            harvestable_by_week: vec![false; SEASON_LENGTH]
        }
    }

    #[cfg(test)]
    pub fn dummy(name: &str, reqs: Vec<&str>) -> Self {
        Variety{
            name: name.to_string(),
            harvest_schedule: vec![],
            planting_schedule: [true;SEASON_LENGTH],
            instructions: std::collections::HashMap::new(),
            requirements: reqs.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            value_per_unit: 100,
            harvestable_by_week: vec![true; SEASON_LENGTH]
        }
    }

    // Get how long the crop lasts from planting out to last harvest
    pub fn get_longevity(&self) -> WeekRange {
        return self.harvest_schedule.len();
    }

    pub fn is_harvestable_in_week(&self, week: usize) -> bool {
        self.harvestable_by_week[week%SEASON_LENGTH]
    }
}