use crate::plan::Plan;
use std::error::Error;
use crate::bed::Bed;
use crate::variety::Variety;
use crate::common::*;
use std::convert::TryFrom;
use json::JsonValue;

// Represents all of the input parameters to the plan building algorithm
// Loaded from params.json
#[derive(Clone)]
pub struct Params {
    pub beds: Vec<Bed>,
    pub varieties: Vec<Variety>,
    pub num_baskets: i32,
    pub plan_previous_year: Plan,
}

impl TryFrom<&JsonValue> for Params {
    type Error = Box<dyn Error>;
    
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        let mut params = Params{
            varieties: vec![],
            beds: vec![],
            num_baskets: 0,
            plan_previous_year: Plan::new(0)
        };

        let value_json_obj = as_object(value)?;
        params.num_baskets = as_int(&value_json_obj["num_baskets"])?;
        let varieties_json_array = as_array(&value_json_obj["varieties"])?;
        params.varieties = varieties_json_array.iter().map(|j| Variety::try_parse(j)).collect::<Result<Vec<_>, _>>()?;
        params.varieties.insert(0, crate::variety::Variety::empty());
        
        let beds_json_array = as_array(&value_json_obj["beds"])?;
        params.beds = beds_json_array.iter().map(|j| Bed::try_from(j)).collect::<Result<Vec<_>, _>>()?;

        match value_json_obj.get("planting_schedule_prior_year") {
            Some(planting_schedule_prior_year_json_obj) => {
                params.plan_previous_year = Plan::try_from(planting_schedule_prior_year_json_obj)?;
            },
            None => params.plan_previous_year = Plan::new(params.beds.len())
        }
        

        Ok(params)
    }
}

impl Params {
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
            "name": "~b00"
        },
        {
            "name": "~b01",
            "properties": [ "polytunnel" ]
        }
    ],
    "varieties": [
        {
            "name": "lettuce",
            "requirements": [ ],
            "harvest_schedule": "0:3,4,5:2",
            "planting_schedule": "3,4,10-20",
            "instructions": {
                "-2": "Seed <variety> into a 144 tray and label it <label>",
                "-1": "Move tray <label> to harden off",
                "0": "Transplant <variety> from tray <label> into bed <bed>"
            },
            "value_per_unit": 100
        },
        {
            "name": "tomato",
            "requirements": [ "polytunnel" ],
            "harvest_schedule": "0:3,4,5:2",
            "planting_schedule": "3,4,10-20",
            "instructions": {
                "-6": "Seed <variety> into a 64 tray and label it <label>",
                "-4": "Transplant <variety> from tray <label> into 20cm pots and label them <label>",
                "0": "Transplant <variety> from pots labelled <label> into bed <bed>"
            },
            "value_per_unit": 100
        }
    ],
    "num_baskets": 42
}"#).expect("test is wrong");

    let params = Params::try_from(&js).expect("failed to parse");

    assert_eq!(params.beds.len(), 2);
    assert_eq!(params.beds[1].name, "~b01");
    assert!(params.beds[1].properties.contains(&String::from("polytunnel")));
    assert!(!params.beds[1].properties.contains(&String::from("magic")));

    assert_eq!(params.varieties.len(), 3);
    assert_eq!(params.varieties[0].name, "");
    assert_eq!(params.varieties[2].name, "tomato");
    assert!(params.varieties[2].requirements.contains(&String::from("polytunnel")));
    assert!(!params.varieties[2].requirements.contains(&String::from("magic")));
    assert_eq!(params.varieties[1].harvest_schedule, vec![0,0,0,4,5,5]);
    assert_eq!(params.varieties[2].instructions["-6"], "Seed <variety> into a 64 tray and label it <label>");
    assert_eq!(params.varieties[2].instructions["0"], "Transplant <variety> from pots labelled <label> into bed <bed>");

    assert_eq!(params.num_baskets, 42);
}

pub const DEFAULT_PARAMS_JSON: &'static str = r#"{
    "beds": [
        {
            "name": "~bA11",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bA12",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bA13",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bA21",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bA22",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bA23",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bA31",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bA32",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bA33",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bA41",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bA42",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bA43",
            "properties": [ "polytunnel" ]
        },

        {
            "name": "~bB11",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bB12",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bB13",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bB21",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bB22",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bB23",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bB31",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bB32",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bB33",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bB41",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bB42",
            "properties": [ "polytunnel" ]
        },
        {
            "name": "~bB43",
            "properties": [ "polytunnel" ]
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
    "num_baskets": 120,
    "varieties": [
        {
            "name": "Lettuce-Indoor",
            "planting_schedule": "0-51",
            "requirements": [ "polytunnel" ],
            "harvest_schedule": "0:8,100:3",
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            },
            "value_per_unit": 45
        },
        {
            "name": "Spinach-Summer",
            "planting_schedule": "9-20",
            "harvest_schedule": "0:8,125:4",
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            },
            "value_per_unit": 160
        },
        {
            "name": "Spinach-Winter",
            "planting_schedule": "40-48",
            "harvest_schedule": "0:12,125:4",
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            },
            "value_per_unit": 160
        },
        {
            "name": "Radish",
            "planting_schedule": "9-45",
            "harvest_schedule": "0:5,50",
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            },
            "value_per_unit": 65
        },
        {
            "name": "Lettuce-Outdoor",
            "planting_schedule": "8-30",
            "harvest_schedule": "0:13,100:3",
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            },
            "value_per_unit": 45
        },
        {
            "name": "Tomato",
            "requirements": [ "polytunnel" ],
            "planting_schedule": "9-18",
            "harvest_schedule": "0:15,120:11",
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            },
            "value_per_unit": 125
        },
        {
            "name": "Carrot-Summer",
            "planting_schedule": "7-14",
            "harvest_schedule": "0:16,25:4",
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            },
            "value_per_unit": 50
        },
        {
            "name": "Carrot-Winter",
            "planting_schedule": "35-45",
            "harvest_schedule": "0:20,25,4",
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            },
            "value_per_unit": 50
        },
        {
            "name": "Swede-Summer",
            "planting_schedule": "16-20",
            "harvest_schedule": "0:16,20:8",
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            },
            "value_per_unit": 80
        },
        {
            "name": "Swede-Winter",
            "planting_schedule": "30-40",
            "harvest_schedule": "0:18,20:8",
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            },
            "value_per_unit": 80
        },
        {
            "name": "BBean",
            "planting_schedule": "20-28",
            "harvest_schedule": "0:24,30",
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            },
            "value_per_unit": 110
        },
        {
            "name": "Brocoli",
            "planting_schedule": "36-42",
            "harvest_schedule": "0:24,22",
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            },
            "value_per_unit": 120
        },
        {
            "name": "SOnion",
            "planting_schedule": "32-44",
            "harvest_schedule": "0:20,120:3",
            "instructions": {
                "-2": "Label a 144 tray <label> and seed it with 6 grams of <variety> seed",
                "-1": "Harden off <variety> tray <label>",
                "0": "Transplant <variety> from tray <label> into bed <bed>",
                "harvest": "Harvest <units> units of <variety> from bed <bed>"
            },
            "value_per_unit": 55
        }
    ]
}"#; 