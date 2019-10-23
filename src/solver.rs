use crate::rand::Rand;
use crate::params::Params;
use crate::solution::{Solution};
use crate::constant::{POPULATION_SIZE, SEASON_LENGTH, FIRST_BOX_WEEK, LAST_BOX_WEEK, NUM_BOXES, SolutionId, GeneId, VarietyId};

pub struct Solver {
    rand: Rand,
    params: Params,
    pop: Vec<Solution>,
}

impl Solver {

    pub fn new(params: Params) -> Solver {
        let rand = Rand::new(&params);
        let pop = vec!(crate::solution::new(&params); POPULATION_SIZE);

        let mut solver = Solver {
            rand: rand,
            params: params,
            pop: pop,
        };

        for i in 0..POPULATION_SIZE {
            solver.rand.randomize_solution(&mut solver.pop[i]);
        }

        // for x in &solver.pop {
        //     for y in x {
        //         print!("{}, ", y);
        //     }
        //     println!("");
        // }

        // initial sort by fitness
        solver.step();

        solver
    }

    fn get_gene(&self, solution: SolutionId, gene: GeneId) -> VarietyId {
        self.pop[solution][gene]
    }

    fn spawn(&mut self, child: &mut Solution) {
        let mother_id = self.rand.select_individual();
        let father_id = self.rand.select_individual();
        self.cross(mother_id, father_id, child);
        self.mutate(child);
    }

    fn score(&self, sol: &Solution) -> i32 {
        let mut score = 0;

        let harvest_plan = crate::solution::to_harvest_plan(sol, &self.params);

        // aim in each week to have the harvestable units of each crop equal to the number of boxes
        for week in 0..SEASON_LENGTH {
            let target_units = if week < FIRST_BOX_WEEK || week > LAST_BOX_WEEK { 0 } else { NUM_BOXES };

            let harvest = &harvest_plan[week];

            for variety_id in 0..self.params.varieties.len() {
                let harvestable_units = harvest[variety_id];

                score -= (harvestable_units - target_units).abs();
            }
        }

        score
    }

    fn cross(&mut self, mother_id: SolutionId, father_id: SolutionId, child: &mut Solution) {
        for gene in 0..self.params.genome_size() {
            let source = match self.rand.random_parent() {
                true => { mother_id }
                false => { father_id }
            };
            let variety = self.get_gene(source, gene);
            child[gene] = variety;
        }
    }

    fn mutate(&mut self, child: &mut Solution) {
        // for _ in 0..3 {
        let gene = self.rand.random_gene();
        let variety = self.rand.random_variety();
        child[gene] = variety;
        // }
    }

    pub fn step(&mut self) {

        let mut next = vec!(crate::solution::new(&self.params); POPULATION_SIZE);

        next[0] = self.get_best_solution().clone();

        // build next generation by selection, crossover and mutation
        for i in 1..POPULATION_SIZE {
            self.spawn(&mut next[i]);
        }

        next.sort_by_cached_key(|p| self.score(p));

        self.pop = next;
    }

    pub fn get_best_score(&self) -> i32 {
        return self.score(&self.pop[POPULATION_SIZE-1]);
    }

    pub fn get_best_solution(&self) -> &Solution {
        self.pop.last().expect("internal error")
    }
}