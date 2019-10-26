use crate::constant::SEASON_LENGTH;

pub struct Instructions{
    content: Vec<Vec<String>>
}

impl Instructions {
    pub fn new() -> Self {
        Instructions{
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