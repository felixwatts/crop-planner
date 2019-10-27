use crate::constant::SEASON_LENGTH;

// Represents the tasks for each week as specified by a crop plan
pub struct Tasks{
    content: Vec<Vec<String>>
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