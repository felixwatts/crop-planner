use std::error::Error;
use json::JsonValue;
use std::convert::TryFrom;
use crate::constant::SEASON_LENGTH;
use crate::common::*;

#[derive(Clone)]
pub struct Resource {
    name: String,
    value_per_unit: u32
}

impl TryFrom<&JsonValue> for Resource {
    type Error = Box<dyn Error>;
    
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        let json_obj = as_object(value)?;
        let name = as_string(&json_obj["name"])?;
        let value_per_unit = as_u32(&json_obj["value_per_unit"])?;
        Ok(Resource{
            name: name,
            value_per_unit: value_per_unit
        })
    }
}

#[cfg(test)]
#[test]
fn resource_from_json() {
    let js = json::parse(r#"
{
    "name": "tomato seed",
    "value_per_unit": 18
}"#).expect("test is wrong");
    let subject = Resource::try_from(&js).expect("failed to parse");
    assert_eq!(subject.name, "tomato seed");
    assert_eq!(subject.value_per_unit, 18);
}

#[derive(Clone, PartialEq, Debug)]
pub struct ResourceRequirement {
    pub name: String,
    pub quantity: u32
}

impl TryFrom<&JsonValue> for ResourceRequirement {
    type Error = Box<dyn Error>;
    
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        let json_obj = as_object(value)?;
        let name = as_string(&json_obj["name"])?;
        let quantity = as_u32(&json_obj["quantity"])?;
        Ok(ResourceRequirement{
            name: name,
            quantity: quantity
        })
    }
}

#[cfg(test)]
#[test]
fn resource_requirement_from_json() {
    let js = json::parse(r#"
{
    "name": "tomato seed",
    "quantity": 12
}"#).expect("test is wrong");
    let subject = ResourceRequirement::try_from(&js).expect("failed to parse");
    assert_eq!(subject.name, "tomato seed");
    assert_eq!(subject.quantity, 12);
}

#[derive(Clone, PartialEq, Debug)]
pub struct Task {
    pub description: String,
    pub resources: Vec<ResourceRequirement>
}

impl TryFrom<&JsonValue> for Task {
    type Error = Box<dyn Error>;
    
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        let json_obj = as_object(value)?;
        let description = as_string(&json_obj["description"])?;
        let resources_arr = as_array(&json_obj["resources"])?;
        let resources = resources_arr.iter().map(|j| ResourceRequirement::try_from(j)).collect::<Result<Vec<_>, _>>()?;
        Ok(Task{
            description: description,
            resources: resources
        })
    }
}

#[cfg(test)]
#[test]
fn task_from_json() {
    let js = json::parse(r#"
{
    "description": "plant a 144 tray with tomato seed",
    "resources": [
        { "name": "tomato seed", "quantity": 6 },
        { "name": "potting mix", "quantity": 1 },
        { "name": "time", "quantity": 300 }
    ]
}"#).expect("test is wrong");
    let subject = Task::try_from(&js).expect("failed to parse");
    assert_eq!(subject.description, "plant a 144 tray with tomato seed");
    assert_eq!(subject.resources, vec![
        ResourceRequirement{ name: String::from("tomato seed"), quantity: 6 },
        ResourceRequirement{ name: String::from("potting mix"), quantity: 1 },
        ResourceRequirement{ name: String::from("time"), quantity: 300 }
    ]);
}

// Represents the tasks for each week as specified by a crop plan
pub struct Tasks{
    content: Vec<Vec<String>>
}

impl TryFrom<&JsonValue> for Tasks {
    type Error = Box<dyn Error>;
    
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        Ok(Tasks{
            content: vec![]
        })
    }
}

impl Tasks {
    pub fn new() -> Self {
        Tasks{
            content: std::iter::repeat(vec![]).take(SEASON_LENGTH).collect()
        }
    }

    pub fn add(&mut self, week: usize, ins: &String) {
        self.content[week].push(ins.clone())
    }

    pub fn get(&self, week: usize) -> &Vec<String> {
        &self.content[week]
    }
}