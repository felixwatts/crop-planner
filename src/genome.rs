use crate::plan::Plan;
use crate::evaluator::Evaluator;
use crate::params::Params;
use crate::constant::SEASON_LENGTH;

// Represents a crop plan encoded as a collection of genes
// Provides methods that are the building blocks of the evolutionary algorithm
#[derive(Clone)]
pub struct Genome<'a> {
    plan: Plan,
    params: &'a Params
}

impl Genome<'_> {
    pub fn new<'a>(params: &'a Params) -> Genome<'a> {
        Genome {
            plan: Plan::new(params.beds.len()),
            params: params
        }
    }

    pub fn to_evaluator(&self) -> Evaluator {
        Evaluator::new(&self.params, &self.plan)
    }

    pub fn cross(mother: &Self, father: &Self, child: &mut Self, rand: &mut crate::rand::Rand) {
        let genes_mother = mother.plan.get_data();
        let genes_father = father.plan.get_data();
        let genes_child = child.plan.get_data_mut();
        for gene in 0..genes_mother.len() {
            let variety = match rand.random_parent() {
                true => { genes_mother[gene] }
                false => { genes_father[gene] }
            };
            genes_child[gene] = variety;
        }
    }

    pub fn mutate(&mut self, rand: &mut crate::rand::Rand) {
        let gene = rand.random_gene();

        let week = gene % SEASON_LENGTH;
        let bed = gene / SEASON_LENGTH;
        let variety = rand.random_variety(week, bed).or(Some(0)).unwrap();

        let end_week = std::cmp::min(SEASON_LENGTH, week+self.params.varieties[variety].get_longevity());

        let genes = self.plan.get_data_mut();

        for w in week..end_week {
            genes[(bed*SEASON_LENGTH)+w] = 0;
        }

        genes[gene] = variety
    }

    pub fn to_plan(&self) -> Plan {
        self.plan.clone()
    }
}