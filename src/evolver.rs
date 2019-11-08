use crate::constant::VarietyId;
use crate::rand::Rand;
use crate::params::Params;
use crate::genome::Genome;
use crate::constant::{POPULATION_SIZE};
use std::io::Write;

// Implements the evolutionary algorithm to find a Genome that represents
// a Phenome with a high score
pub struct Evolver<'a> {
    rand: Rand,
    params: &'a Params,
    pop: Vec<Genome<'a>>,
}

impl<'a> Evolver<'a> {

    pub fn new(params: &'a Params) -> Evolver<'a> {
        let rand = Rand::new(&params);
        let pop = vec!(Genome::new(&params); POPULATION_SIZE);

        Evolver {
            rand: rand,
            params: params,
            pop: pop,
        }
    }

    pub fn solve(&mut self) -> Vec<VarietyId> {
        let mut num_gens_without_improvement = 0;
        let mut best_fitness = std::i32::MIN;
        while num_gens_without_improvement < 1000 {
            self.step();
            let fitness = self.get_best_solution().to_evaluator().get_profit();
            if fitness > best_fitness {
                best_fitness = fitness;
                num_gens_without_improvement = 0;

                print!(".");
                std::io::stdout().flush().expect("internal error");
            } else {
                num_gens_without_improvement += 1;
            }
        }

        println!();

        self.get_best_solution().get_genes()     
    }

    pub fn step(&mut self) {
        let mut next_population = self.get_new_population();
        self.spawn_next_generation_into(&mut next_population);
        self.sort_by_fitness(&mut next_population);
        self.pop = next_population;
    }

    pub fn get_best_solution(&self) -> &Genome<'a> {
        self.pop.last().unwrap()
    }

    fn spawn_next_generation_into(&mut self, next: &mut Vec<Genome<'a>>) {
        // Elitism, best individual survives unchanged, means that best score can never decrease
        next[0] = self.get_best_solution().clone();

        for i in 1..POPULATION_SIZE {
            self.spawn(&mut next[i]);
        }
    }

    fn spawn(&mut self, child: &mut Genome<'a>) {
        // Selection
        let mother_id = self.rand.select_individual();
        let father_id = self.rand.select_individual();

        // Crossover
        Genome::cross(&self.pop[mother_id], &self.pop[father_id], child, &mut self.rand);

        // Mutation
        child.mutate(&mut self.rand);
    }

    fn get_new_population(&self) -> Vec<Genome<'a>> {
        vec!(crate::genome::Genome::new(&self.params); POPULATION_SIZE)
    }

    fn sort_by_fitness(&mut self, population: &mut Vec<Genome<'a>>) {
        population.sort_by_cached_key(|p| p.to_evaluator().get_profit());
    }
}