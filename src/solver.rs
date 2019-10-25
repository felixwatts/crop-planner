use crate::rand::Rand;
use crate::params::Params;
use crate::genome::Genome;
use crate::constant::{POPULATION_SIZE, SEASON_LENGTH, NUM_BOXES, SolutionId, GeneId, VarietyId};

pub struct Solver<'a> {
    rand: Rand,
    params: &'a Params,
    pop: Vec<Genome>,
}

impl Solver<'_> {

    pub fn new<'a>(params: &'a Params) -> Solver<'a> {
        let rand = Rand::new(&params);
        let pop = vec!(Genome::new(&params); POPULATION_SIZE);

        let mut solver = Solver {
            rand: rand,
            params: params,
            pop: pop,
        };

        for i in 0..POPULATION_SIZE {
            solver.pop[i].randomize(&mut solver.rand);
        }

        // initial sort by fitness
        solver.step();

        solver
    }

    fn spawn(&mut self, child: &mut Genome) {
        let mother_id = self.rand.select_individual();
        let father_id = self.rand.select_individual();
        Genome::cross(&self.pop[mother_id], &self.pop[father_id], child, &mut self.rand);
        child.mutate(&mut self.rand);
    }

    pub fn step(&mut self) {

        let mut next = vec!(crate::genome::Genome::new(&self.params); POPULATION_SIZE);

        next[0] = self.get_best_solution().clone();

        // build next generation by selection, crossover and mutation
        for i in 1..POPULATION_SIZE {
            self.spawn(&mut next[i]);
        }

        next.sort_by_cached_key(|p| p.score(&self.params));

        self.pop = next;
    }

    pub fn get_best_score(&self) -> i32 {
        self.pop[POPULATION_SIZE-1].score(&self.params)
    }

    pub fn get_best_solution(&self) -> &Genome {
        self.pop.last().unwrap()
    }
}