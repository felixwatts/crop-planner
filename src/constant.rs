pub type VarietyId = usize;
pub type GeneId = usize;
pub type SolutionId = usize;
pub type WeekId = usize;
pub type WeekRange = usize;
pub type HarvestableUnits = i32;

pub const NUM_BEDS: usize = 100;
pub const NUM_BOXES: i32 = 120;
pub const SEASON_LENGTH: WeekRange = 52;
pub const FIRST_BOX_WEEK: WeekId = 14;
pub const LAST_BOX_WEEK: WeekId = 42;
pub const POPULATION_SIZE: usize = 50;
pub const NUM_VARIETIES: usize = 7;
pub const SOLUTION_SIZE: usize = NUM_BEDS * SEASON_LENGTH;

