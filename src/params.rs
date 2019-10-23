use crate::bed::{Bed, BED_FLAG_NONE, BED_FLAG_POLYTUNNEL};
use crate::variety::Variety;
use crate::constant::SEASON_LENGTH;
use crate::common::*;
use std::convert::TryFrom;
use json::JsonValue;

#[derive(Clone)]
pub struct Params {
    pub beds: Vec<Bed>,
    pub varieties: Vec<Variety>
}

impl TryFrom<&JsonValue> for Params {
    type Error = &'static str;
    
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {

        let value_json_obj = as_object(value)?;
        let varieties_json_array = as_array(&value_json_obj["varieties"])?;
        let mut varieties = varieties_json_array.iter().map(|j| Variety::try_from(j)).collect::<Result<Vec<_>, _>>()?;
        varieties.insert(0, Variety{
            name: String::from(""),
            flags: BED_FLAG_NONE,
            harvest_schedule: vec![ 0 ],
            planting_schedule: [ true; SEASON_LENGTH ]
        });
        let beds_json_array = as_array(&value_json_obj["beds"])?;
        let beds = beds_json_array.iter().map(|j| Bed::try_from(j)).collect::<Result<Vec<_>, _>>()?;

        Ok(Params {
            varieties: varieties,
            beds: beds
        })
    }
}

impl Params {
    pub fn genome_size(&self) -> usize {
        self.beds.len() * SEASON_LENGTH
    }

    pub fn num_beds(&self) -> usize {
        self.beds.len()
    }
}

#[cfg(test)]
#[test]
fn params_from_json() {
    let js = json::parse(r#"
{
    "beds": [
        {
            "name": "~b00",
            "flags": [  ]
        },
        {
            "name": "~b01",
            "flags": [ "polytunnel" ]
        }
    ],
    "varieties": [
        {
            "name": "lettuce",
            "flags": [ ],
            "harvest_schedule": [ 0, 1, 2, 3 ],
            "planting_schedule": "apr,may"
        },
        {
            "name": "tomato",
            "flags": [ "polytunnel" ],
            "harvest_schedule": [ 0, 1, 2, 3 ],
            "planting_schedule": "apr,may"
        }
    ]
}"#).expect("test is wrong");
    let params = Params::try_from(&js).expect("failed to parse");
    assert_eq!(params.beds.len(), 2);
    assert_eq!(params.beds[1].name, "~b01");
    assert!(params.beds[1].flags.has_all(&BED_FLAG_POLYTUNNEL));

    assert_eq!(params.varieties.len(), 3);
    assert_eq!(params.varieties[0].name, "");
    assert_eq!(params.varieties[2].name, "tomato");
    assert!(params.varieties[2].flags.has_all(&BED_FLAG_POLYTUNNEL));
    assert_eq!(params.varieties[1].harvest_schedule.len(), 4);
    assert_eq!(params.varieties[1].harvest_schedule[2], 2);
}