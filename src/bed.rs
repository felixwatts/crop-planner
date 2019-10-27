use std::convert::TryFrom;
use json::JsonValue;
use crate::common::*;

// Definition of a single bed, as loaded from params.json
#[derive(Clone, Debug)]
pub struct Bed {
    pub name: String,
    pub properties: Vec<String>,
}

impl TryFrom<&JsonValue> for Bed {
    type Error = &'static str;
    fn try_from(item: &JsonValue) -> Result<Self, Self::Error> {
        let name = as_string(&item["name"])?;
        let properties = match &item["properties"] {
            JsonValue::Array(arr) => arr.iter().map(|p| as_string(p)).collect::<Result<Vec<_>,_>>(),
            _ => Ok(vec![])
        }?;
        Ok(Bed {
            name: String::from(name),
            properties: properties,
        })
    }
}

#[cfg(test)]
#[test]
fn bed_from_json() {
    let js = json::parse(r#"
{
    "name": "~b00",
    "properties": [ "prop1", "prop2" ]
}"#).expect("test is wrong");
    let bed = Bed::try_from(&js).expect("failed to parse");
    assert_eq!(bed.name, "~b00");
    assert!(bed.properties.contains(&String::from("prop1")));
    assert!(bed.properties.contains(&String::from("prop2")));
    assert!(!bed.properties.contains(&String::from("prop3")));
}

