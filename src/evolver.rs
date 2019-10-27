use crate::rand::Rand;
use crate::params::Params;
use crate::genome::Genome;
use crate::constant::{POPULATION_SIZE};

// Implements the evolutionary algorithm to find a Genome that represents
// a Phenome with a high score
pub struct Evolver<'a> {
    rand: Rand,
    params: &'a Params,
    pop: Vec<Genome<'a>>,
}

impl<'a> Evolver<'a> {

    pub fn new<'b>(params: &'b Params) -> Evolver<'b> {
        let rand = Rand::new(&params);
        let pop = vec!(Genome::new(&params); POPULATION_SIZE);

        let mut evolver = Evolver {
            rand: rand,
            params: params,
            pop: pop,
        };

        for i in 0..POPULATION_SIZE {
            evolver.pop[i].randomize(&mut evolver.rand);
        }

        // initial sort by fitness
        evolver.step();

        evolver
    }

    fn spawn(&mut self, child: &mut Genome<'a>) {
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

        next.sort_by_cached_key(|p| p.to_phenome().score());

        self.pop = next;
    }

    // pub fn get_best_score(&self) -> i32 {
    //     self.pop[POPULATION_SIZE-1].to_phenome().score()
    // }

    pub fn get_best_solution(&self) -> &Genome<'a> {
        self.pop.last().unwrap()
    }
}