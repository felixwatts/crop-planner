use crate::common::as_usize;
use crate::common::as_array;
use std::error::Error;
use std::convert::TryFrom;
use json::JsonValue;
use crate::constant::SEASON_LENGTH;
use crate::constant::VarietyId;

#[derive(Clone, Debug)]
pub struct Plan {
    data: Vec<VarietyId>
}

impl Plan {
    pub fn new(num_beds: usize) -> Self {
        Plan {
            data: vec![0; SEASON_LENGTH*num_beds]
        }
    }

    pub fn get(&self, bed: usize, week: usize) -> VarietyId {
        self.data[bed*SEASON_LENGTH+week]
    }

    pub fn get_data(&self) -> &[VarietyId] {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut [VarietyId] {
        &mut self.data
    }

    pub fn to_json(&self) -> JsonValue {
        json::from(self.data.clone())
    }

    pub fn get_last_planting_week(&self, bed: usize) -> Option<usize> {
        self.data[bed*SEASON_LENGTH..bed*SEASON_LENGTH+SEASON_LENGTH].iter().rposition(|&x| x != 0)
    }

    pub fn get_num_plantings(&self) -> i32 {
        self.data.iter().map(|x| match x { 0 => 0i32, _ => 1i32 }).sum()
    }
}

impl TryFrom<&JsonValue> for Plan {
    type Error = Box<dyn Error>;
    
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        let json_arr = as_array(value)?;
        let usize_arr = json_arr.iter().map(|i| as_usize(i)).collect::<Result<Vec<_>, _>>()?;
        Ok(Plan{
            data: usize_arr
        })
    }
}