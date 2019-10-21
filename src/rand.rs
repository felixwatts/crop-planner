use rand::rngs::ThreadRng;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::distributions::WeightedIndex;
use rand::distributions::Bernoulli;
use crate::solution::Solution;
use crate::constant::{POPULATION_SIZE, SOLUTION_SIZE, NUM_VARIETIES, GeneId, VarietyId, SolutionId};

pub struct Rand {
    rng: ThreadRng,
    dist_gene: Uniform<GeneId>,
    dist_variety: Uniform<VarietyId>,
    dist_selection: WeightedIndex<SolutionId>,
    dist_parent: Bernoulli
}

pub fn new_rand() -> Rand {
    let weights = 1..(POPULATION_SIZE+1);

    return Rand{
        rng: rand::thread_rng(),
        dist_gene: Uniform::from(0..(SOLUTION_SIZE)),
        dist_variety: Uniform::from(0..(NUM_VARIETIES)),
        dist_selection: WeightedIndex::new(weights).unwrap(),
        dist_parent: Bernoulli::new(0.5).unwrap()
    }
}

impl Rand {
    pub fn random_parent(&mut self) -> bool {
        return self.dist_parent.sample(&mut self.rng);
    }

    pub fn random_gene(&mut self) -> GeneId {
        return self.dist_gene.sample(&mut self.rng);
    }

    pub fn random_variety(&mut self) -> VarietyId {
        return self.dist_variety.sample(&mut self.rng);
    }

    pub fn select_individual(&mut self) -> SolutionId {
        return self.dist_selection.sample(&mut self.rng);
    }

    pub fn randomize_solution(&mut self, slot: &mut Solution) {
        for gene in 0..slot.len() {
            slot[gene] = self.random_variety();
        }
    }
}