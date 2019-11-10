use rand::rngs::ThreadRng;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::distributions::WeightedIndex;
use rand::distributions::Bernoulli;
use crate::params::Params;
use crate::constant::{POPULATION_SIZE, SEASON_LENGTH, GeneId, VarietyId, SolutionId};

// Provides randomization methods for the evolutionary algorithm
pub struct Rand {
    rng: ThreadRng,
    dist_gene: Uniform<GeneId>,
    dist_selection: WeightedIndex<SolutionId>,
    dist_parent: Bernoulli,
    plantable_varieties_by_week_by_bed: Vec<Vec<Vec<VarietyId>>>,
    dist_plantable_variety_by_week_by_bed: Vec<Vec<Option<Uniform<VarietyId>>>>,
}

impl Rand {
    pub fn new(params: &Params) -> Rand {
        let weights = 1..(POPULATION_SIZE+1);

        let mut plantable_varieties_by_week_by_bed = Vec::<Vec<Vec<VarietyId>>>::new();
        let mut dist_plantable_variety_by_week_by_bed = Vec::<Vec<Option<Uniform<VarietyId>>>>::new();

        for bed in 0..params.beds.len() {
            let mut plantable_varieties_by_week = std::iter::repeat(vec![ ])
                .take(SEASON_LENGTH)
                .collect::<Vec<_>>();

            for week in 0..SEASON_LENGTH {
                for variety in 0..params.varieties.len() {
                    if params.varieties[variety].planting_schedule[week] {

                        if params.varieties[variety].are_requirements_met(&params.beds[bed]) {
                            plantable_varieties_by_week[week].push(variety);
                        }
                    }
                }
            }

            let dist_plantable_variety_by_week = plantable_varieties_by_week
                .iter()
                .map(|w| if w.len() == 0 { None } else { Some(Uniform::from(0..w.len())) })
                .collect::<Vec<_>>();

            plantable_varieties_by_week_by_bed.push(plantable_varieties_by_week);
            dist_plantable_variety_by_week_by_bed.push(dist_plantable_variety_by_week);
        }

        return Rand{
            rng: rand::thread_rng(),
            dist_gene: Uniform::from(0..params.beds.len()*SEASON_LENGTH),
            dist_selection: WeightedIndex::new(weights).unwrap(),
            dist_parent: Bernoulli::new(0.5).unwrap(),
            plantable_varieties_by_week_by_bed: plantable_varieties_by_week_by_bed,
            dist_plantable_variety_by_week_by_bed: dist_plantable_variety_by_week_by_bed
        }
    }

    // Choose one of two parents with equal probability
    pub fn random_parent(&mut self) -> bool {
        return self.dist_parent.sample(&mut self.rng);
    }

    // Choose a gene from the genome at random with uniform probability
    pub fn random_gene(&mut self) -> GeneId {
        return self.dist_gene.sample(&mut self.rng);
    }

    // Choose an individual from the fitness sorted population, with
    // likelihood of selection proportional to fitness
    pub fn select_individual(&mut self) -> SolutionId {
        return self.dist_selection.sample(&mut self.rng);
    }

    // Choose a variety from the set of varieties that can be planted in
    // the given bed at the given week, with uniform probability
    pub fn random_variety(&mut self, week: usize, bed: usize) -> Option<VarietyId> {
        match self.dist_plantable_variety_by_week_by_bed[bed][week] {
            Some(dist) => {
                let i = dist.sample(&mut self.rng);
                Some(self.plantable_varieties_by_week_by_bed[bed][week][i])
            },
            None => None
        }
    }
}

#[cfg(test)]
#[test]
fn random_variety_meets_requirements() {
    let params = Params{
        beds: vec![ crate::bed::Bed{
            name: "bed-0".to_string(),
            properties: vec![ "rq-1".to_string() ]
        } ],
        varieties: vec![ 
            crate::variety::Variety::dummy("var-0", vec![]),
            crate::variety::Variety::dummy("var-1", vec!["rq-1"]),
            crate::variety::Variety::dummy("var-2", vec!["rq-2"]),
            crate::variety::Variety::dummy("var-2", vec!["rq-1", "rq-2"]),
        ],
        num_baskets: 120,
        planting_schedule_prior_year: vec![0; 1 * SEASON_LENGTH]
    };

    let mut subject = Rand::new(&params);

    for _ in 0..100 {
        for w in 0..SEASON_LENGTH {
            let v = subject.random_variety(w, 0).expect("fail");
            assert_ne!(v, 2);
            assert_ne!(v, 3);
        }
    }
}

#[cfg(test)]
#[test]
fn random_variety_satisfies_planting_schedule() {
    let mut params = Params{
        beds: vec![ crate::bed::Bed{
            name: "bed-0".to_string(),
            properties: vec![ ]
        } ],
        varieties: vec![ crate::variety::Variety::dummy("var-0", vec![]) ],
        num_baskets: 120,
        planting_schedule_prior_year: vec![0; 1 * SEASON_LENGTH]
    };

    for i in 0..SEASON_LENGTH {
        params.varieties[0].planting_schedule[i] = i % 2 == 0;
    }

    let mut subject = Rand::new(&params);

    for _ in 0..100 {
        for w in 0..SEASON_LENGTH {
            let v = subject.random_variety(w, 0);
            
            match v {
                Some(_) => assert_eq!(w%2, 0),
                None => assert_ne!(w%2, 0)
            };
        }
    }
}

#[cfg(test)]
#[test]
fn select_individual() {
    let params = Params{
        beds: vec![ crate::bed::Bed{
            name: "bed-1".to_string(),
            properties: vec![]
        } ],
        varieties: vec![ crate::variety::Variety::dummy("var-1", vec![]) ],
        num_baskets: 120,
        planting_schedule_prior_year: vec![0; 1 * SEASON_LENGTH]
    };
    let mut subject = Rand::new(&params);

    let mut count_low = 0;
    let mut count_hi = 0;
    for _ in 0..1000 {
        let i = subject.select_individual();
        assert!(i < POPULATION_SIZE);
        if i < 10 {
            count_low += 1
        }
        if i > POPULATION_SIZE-10 {
            count_hi += 1
        }
    }
    assert!(count_low < count_hi)
}