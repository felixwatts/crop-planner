use core::fmt::Display;
use crate::tasks::Tasks;
use crate::constant::VarietyId;
use crate::params::Params;
use crate::bed_plan::BedPlan;
use std::convert::TryInto;

pub struct Formatter<'a> {
    params: &'a Params,
    planting_schedule: &'a Vec<VarietyId>
}

impl<'a> Formatter<'a> {
    pub fn new(params: &'a Params, planting_schedule: &'a Vec<VarietyId>) -> Formatter<'a> {
        Formatter{
            params: params,
            planting_schedule: planting_schedule
        }
    }
}

impl<'a> Display for Formatter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}