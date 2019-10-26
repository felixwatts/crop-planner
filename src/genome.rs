use crate::phenome::Phenome;
use crate::params::Params;
use crate::constant::{ SEASON_LENGTH, VarietyId };


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

    pub fn randomize(&mut self, rand: &mut crate::rand::Rand) {
        for gene in 0..self.genes.len() {
            self.randomize_gene(gene, rand);
        }
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
        self.randomize_gene(gene, rand);
    }

    pub fn get_genes(&self) -> Vec<usize> {
        self.genes.clone()
    }

    fn randomize_gene(&mut self, gene: usize, rand: &mut crate::rand::Rand) {
        let week = gene % SEASON_LENGTH;
        let bed = gene / SEASON_LENGTH;
        self.genes[gene] = rand.random_variety(week, bed).or(Some(0)).unwrap();
    }
}

impl std::convert::Into<json::JsonValue> for Genome<'_> {
    fn into(self) -> json::JsonValue {
        self.genes.clone().into()
    }
}