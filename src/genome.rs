use crate::phenome::Phenome;
use crate::params::Params;
use crate::constant::{ SEASON_LENGTH, VarietyId };

// Represents a crop plan encoded as a collection of genes
// Provides methods that are the building blocks of the evolutionary algorithm
#[derive(Clone)]
pub struct Genome<'a> {
    genes: Vec<VarietyId>,
    params: &'a Params
}

impl Genome<'_> {
    pub fn new<'a>(params: &'a Params) -> Genome<'a> {
        Genome {
            genes: vec![ 0; params.genome_size() ],
            params: params
        }
    }

    pub fn from_genes<'a>(genes: &Vec<usize>, params: &'a Params) -> Genome<'a> {
        Genome {
            genes: genes.clone(),
            params: params
        }
    }

    pub fn to_phenome(&self) -> Phenome {
        Phenome::new(&self.genes, &self.params)
    }

    pub fn cross(mother: &Self, father: &Self, child: &mut Self, rand: &mut crate::rand::Rand) {
        for gene in 0..mother.genes.len() {
            let variety = match rand.random_parent() {
                true => { mother.genes[gene] }
                false => { father.genes[gene] }
            };
            child.genes[gene] = variety;
        }
    }

    pub fn mutate(&mut self, rand: &mut crate::rand::Rand) {
        let gene = rand.random_gene();

        let week = gene % SEASON_LENGTH;
        let bed = gene / SEASON_LENGTH;
        let variety = rand.random_variety(week, bed).or(Some(0)).unwrap();

        for w in week..week+self.params.varieties[variety].get_longevity() {
            self.genes[(bed*SEASON_LENGTH)+(w%SEASON_LENGTH)] = 0;
        }

        self.genes[gene] = variety
    }

    pub fn get_genes(&self) -> Vec<usize> {
        self.genes.clone()
    }
}

impl std::convert::Into<json::JsonValue> for Genome<'_> {
    fn into(self) -> json::JsonValue {
        self.genes.clone().into()
    }
}