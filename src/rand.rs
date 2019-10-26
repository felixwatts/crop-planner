use rand::rngs::ThreadRng;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::distributions::WeightedIndex;
use rand::distributions::Bernoulli;
use crate::params::Params;
use crate::constant::{POPULATION_SIZE, SEASON_LENGTH, GeneId, VarietyId, SolutionId};

pub struct Rand {
    rng: ThreadRng,
    dist_gene: Uniform<GeneId>,
    dist_selection: WeightedIndex<SolutionId>,
    dist_parent: Bernoulli,
    plantable_varieties_by_week: Vec<Vec<VarietyId>>,
    dist_plantable_variety_by_week: Vec<Uniform<VarietyId>>,
}

impl Rand {
    pub fn new(params: &Params) -> Rand {
        let weights = 1..(POPULATION_SIZE+1);
        let mut plantable_varieties_by_week = std::iter::repeat(vec![ ])
            .take(SEASON_LENGTH)
            .collect::<Vec<_>>();
        for week in 0..SEASON_LENGTH {
            for variety in 0..params.varieties.len() {
                if params.varieties[variety].planting_schedule[week] {
                    plantable_varieties_by_week[week].push(variety);
                }
            }
        } 
        let dist_plantable_variety_by_week = plantable_varieties_by_week
            .iter()
            .map(|w| Uniform::from(0..w.len()))
            .collect::<Vec<_>>();

        return Rand{
            rng: rand::thread_rng(),
            dist_gene: Uniform::from(0..params.genome_size()),
            dist_selection: WeightedIndex::new(weights).unwrap(),
            dist_parent: Bernoulli::new(0.5).unwrap(),
            plantable_varieties_by_week: plantable_varieties_by_week,
            dist_plantable_variety_by_week: dist_plantable_variety_by_week
        }
    }

    pub fn random_parent(&mut self) -> bool {
        return self.dist_parent.sample(&mut self.rng);
    }

    pub fn random_gene(&mut self) -> GeneId {
        return self.dist_gene.sample(&mut self.rng);
    }

    pub fn select_individual(&mut self) -> SolutionId {
        return self.dist_selection.sample(&mut self.rng);
    }

    pub fn random_variety(&mut self, week: usize) -> VarietyId {
        let i = self.dist_plantable_variety_by_week[week].sample(&mut self.rng);
        self.plantable_varieties_by_week[week][i]
    }
}