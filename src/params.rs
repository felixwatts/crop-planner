use crate::bed::{Bed, BED_FLAG_POLYTUNNEL};
use crate::variety::Variety;
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
        let varieties = varieties_json_array.iter().map(|j| Variety::try_from(j)).collect::<Result<Vec<_>, _>>()?;
        let beds_json_array = as_array(&value_json_obj["beds"])?;
        let beds = beds_json_array.iter().map(|j| Bed::try_from(j)).collect::<Result<Vec<_>, _>>()?;

        Ok(Params {
            varieties: varieties,
            beds: beds
        })
    }
}

#[cfg(test)]
#[test]
fn variety_from_json() {
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
            "flags": [ ]
        },
        {
            "name": "tomato",
            "flags": [ "polytunnel" ]
        }
    ]
}"#).expect("test is wrong");
    let params = Params::try_from(&js).expect("failed to parse");
    assert_eq!(params.beds.len(), 2);
    assert_eq!(params.beds[1].name, "~b01");
    assert!(params.beds[1].flags.has_all(&BED_FLAG_POLYTUNNEL));

    assert_eq!(params.varieties.len(), 2);
    assert_eq!(params.varieties[1].name, "tomato");
    assert!(params.varieties[1].flags.has_all(&BED_FLAG_POLYTUNNEL));
}