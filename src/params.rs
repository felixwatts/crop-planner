use std::error::Error;
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
    type Error = Box<dyn Error>;
    
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {

        let value_json_obj = as_object(value)?;
        let varieties_json_array = as_array(&value_json_obj["varieties"])?;
        let mut varieties = varieties_json_array.iter().map(|j| Variety::try_from(j)).collect::<Result<Vec<_>, _>>()?;
        varieties.insert(0, Variety{
            name: String::from(""),
            flags: BED_FLAG_NONE,
            harvest_schedule: vec![ 0 ],
            planting_schedule: [ true; SEASON_LENGTH ],
            instructions: std::collections::HashMap::new()
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

    pub fn get_bed(&self, name: &str) -> Option<usize> {
        self.beds.iter().position(|b| b.name == name)
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
            "planting_schedule": "apr,may",
            "instructions": {
                "-2": "Seed <variety> into a 144 tray and label it <label>",
                "-1": "Move tray <label> to harden off",
                "0": "Transplant <variety> from tray <label> into bed <bed>"
            }
        },
        {
            "name": "tomato",
            "flags": [ "polytunnel" ],
            "harvest_schedule": [ 0, 1, 2, 3 ],
            "planting_schedule": "apr,may",
            "instructions": {
                "-6": "Seed <variety> into a 64 tray and label it <label>",
                "-4": "Transplant <variety> from tray <label> into 20cm pots and label them <label>",
                "0": "Transplant <variety> from pots labelled <label> into bed <bed>"
            }
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
    assert_eq!(params.varieties[2].instructions["-6"], "Seed <variety> into a 64 tray and label it <label>");
    assert_eq!(params.varieties[2].instructions["0"], "Transplant <variety> from pots labelled <label> into bed <bed>");
}

pub const DEFAULT_PARAMS_JSON: &'static str = r#"{
    "beds": [
        {
            "name": "~bA11",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bA12",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bA13",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bA21",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bA22",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bA23",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bA31",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bA32",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bA33",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bA41",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bA42",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bA43",
            "flags": [ "polytunnel" ]
        },

        {
            "name": "~bB11",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bB12",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bB13",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bB21",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bB22",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bB23",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bB31",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bB32",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bB33",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bB41",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bB42",
            "flags": [ "polytunnel" ]
        },
        {
            "name": "~bB43",
            "flags": [ "polytunnel" ]
        },

        {
            "name": "~bC11"
        },
        {
            "name": "~bC12"
        },
        {
            "name": "~bC13"
        },
        {
            "name": "~bC21"
        },
        {
            "name": "~bC22"
        },
        {
            "name": "~bC23"
        },
        {
            "name": "~bC31"
        },
        {
            "name": "~bC32"
        },
        {
            "name": "~bC33"
        },
        {
            "name": "~bC41"
        },
        {
            "name": "~bC42"
        },
        {
            "name": "~bC43"
        },

        {
            "name": "~bD11"
        },
        {
            "name": "~bD12"
        },
        {
            "name": "~bD13"
        },
        {
            "name": "~bD21"
        },
        {
            "name": "~bD22"
        },
        {
            "name": "~bD23"
        },
        {
            "name": "~bD31"
        },
        {
            "name": "~bD32"
        },
        {
            "name": "~bD33"
        },
        {
            "name": "~bD41"
        },
        {
            "name": "~bD42"
        },
        {
            "name": "~bD43"
        },

        {
            "name": "~bE11"
        },
        {
            "name": "~bE12"
        },
        {
            "name": "~bE13"
        },
        {
            "name": "~bE21"
        },
        {
            "name": "~bE22"
        },
        {
            "name": "~bE23"
        },
        {
            "name": "~bE31"
        },
        {
            "name": "~bE32"
        },
        {
            "name": "~bE33"
        },
        {
            "name": "~bE41"
        },
        {
            "name": "~bE42"
        },
        {
            "name": "~bE43"
        },

        {
            "name": "~bF11"
        },
        {
            "name": "~bF12"
        },
        {
            "name": "~bF13"
        },
        {
            "name": "~bF21"
        },
        {
            "name": "~bF22"
        },
        {
            "name": "~bF23"
        },
        {
            "name": "~bF31"
        },
        {
            "name": "~bF32"
        },
        {
            "name": "~bF33"
        },
        {
            "name": "~bF41"
        },
        {
            "name": "~bF42"
        },
        {
            "name": "~bF43"
        },

        {
            "name": "~bG11"
        },
        {
            "name": "~bG12"
        },
        {
            "name": "~bG13"
        },
        {
            "name": "~bG21"
        },
        {
            "name": "~bG22"
        },
        {
            "name": "~bG23"
        },
        {
            "name": "~bG31"
        },
        {
            "name": "~bG32"
        },
        {
            "name": "~bG33"
        },
        {
            "name": "~bG41"
        },
        {
            "name": "~bG42"
        },
        {
            "name": "~bG43"
        },

        {
            "name": "~bH11"
        },
        {
            "name": "~bH12"
        },
        {
            "name": "~bH13"
        },
        {
            "name": "~bH21"
        },
        {
            "name": "~bH22"
        },
        {
            "name": "~bH23"
        },
        {
            "name": "~bH31"
        },
        {
            "name": "~bH32"
        },
        {
            "name": "~bH33"
        },
        {
            "name": "~bH41"
        },
        {
            "name": "~bH42"
        },
        {
            "name": "~bH43"
        },

        {
            "name": "~bI11"
        },
        {
            "name": "~bI12"
        },
        {
            "name": "~bI13"
        },
        {
            "name": "~bI21"
        },
        {
            "name": "~bI22"
        },
        {
            "name": "~bI23"
        },
        {
            "name": "~bI31"
        },
        {
            "name": "~bI32"
        },
        {
            "name": "~bI33"
        },
        {
            "name": "~bI41"
        },
        {
            "name": "~bI42"
        },
        {
            "name": "~bI43"
        }
    ],
    "varieties": [
        {
            "name": "Spinach",
            "planting_schedule": "mar,apr,may,jun,aug",
            "harvest_schedule": [ 0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8 ],
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            }
        },
        {
            "name": "Radish",
            "planting_schedule": "mar,apr,may,jun,jul,aug,sep",
            "harvest_schedule": [ 0, 0, 0, 0, 0, 20 ],
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            }
        },
        {
            "name": "Lettuce",
            "planting_schedule": "mar,apr,may,jun,jul",
            "harvest_schedule": [ 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5 ],
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            }
        },
        {
            "name": "Tomato",
            "flags": [ "polytunnel" ],
            "planting_schedule": "mar,apr",
            "harvest_schedule": [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10 ],
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            }
        },
        {
            "name": "Carrot",
            "planting_schedule": "mar,apr,may,jun,jul",
            "harvest_schedule": [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 30, 30, 30 ],
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            }
        },
        {
            "name": "Swede",
            "planting_schedule": "apr,may,jun",
            "harvest_schedule": [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10 ],
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            }
        },
        {
            "name": "BBean",
            "planting_schedule": "may,jun",
            "harvest_schedule": [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10 ],
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            }
        },
        {
            "name": "Brocoli",
            "planting_schedule": "sep,oct",
            "harvest_schedule": [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10 ],
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            }
        },
        {
            "name": "SOnion",
            "planting_schedule": "aug,sep,oct",
            "harvest_schedule": [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10 ],
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            }
        }
    ]
}"#;