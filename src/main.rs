
extern crate rand;

use rand::Rng;
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

type VarietyId = usize;
type WeekId = usize;
type WeekRange = usize;
type HarvestableUnits = i32;

const NUM_BEDS: usize = 80;
const NUM_BOXES: i32 = 120;
const MAX_CROPS_PER_BED: usize = 16;
const SEASON_LENGTH: WeekRange = 24;
const FIRST_BOX_WEEK: WeekId = 4;
const POPULATION_SIZE: usize = 100;

struct Variety<'a> {
    name: &'a str,
    longevity: WeekRange,
    growth_model: [ HarvestableUnits; SEASON_LENGTH ],
}

const NUM_VARIETIES: usize = 6;

const VARIETIES: [Variety; NUM_VARIETIES] = [ 
    Variety{
        name: "         ",
        longevity: 1,
        growth_model: [ 0; 24 ],
    }, 
    Variety{
        name: "Spinach  ",
        longevity: 23,
        growth_model: [ 0, 0, 0, 0, 0, 0, 2, 4, 6, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 4, 2, 0 ],
    }, 
    Variety{
        name: "Radish   ",
        longevity: 2,
        growth_model: [ 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    }, 
    Variety{
        name: "Lettuce  ",
        longevity: 4,
        growth_model: [ 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    }, 
    Variety{
        name: "Tomato   ",
        longevity: 14,
        growth_model: [ 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    }, 
    Variety{
        name: "Carrot   ",
        longevity: 16,
        growth_model: [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 12, 12, 12, 10, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0 ],
    }, 
];

type Solution = [VarietyId; NUM_BEDS * MAX_CROPS_PER_BED];

fn get_random<R: Rng + ?Sized>(rng: &mut R) -> Solution {
    let mut solution:Solution = [0; NUM_BEDS * MAX_CROPS_PER_BED];

    for gene in 0..solution.len() {
        solution[gene] = rng.gen_range(0, NUM_VARIETIES); // TODO can be made faster see docs
    }

    return solution;
}

fn mutate<R: Rng + ?Sized>(solution: &mut Solution, rng: &mut R) {
    // let mut mutated = solution.clone();

    for _ in 0..3 {
        let index = rng.gen_range(0, solution.len()); // TODO can be made faster see docs
        let value = rng.gen_range(0, NUM_VARIETIES); // TODO can be made faster see docs
        solution[index] = value;
    }

    // return mutated;
}

fn get_crossed<R: Rng + ?Sized>(mother: &Solution, father: &Solution, rng: &mut R) -> Solution {
    let mut child = mother.clone();
    for gene in 0..child.len() {
        if rng.gen_bool(0.5) { // TODO can be made faster see docs
            child[gene] = father[gene];
        }
    }
    return child;
}

fn optimize(sol: &mut Solution) {
    for bed in 0..NUM_BEDS {
        let mut week = 0;
        for crop in 0..MAX_CROPS_PER_BED {
            let gene_id = bed * MAX_CROPS_PER_BED + crop;
            if week >= SEASON_LENGTH {
                sol[gene_id] = 0
            } else {
                let variety_id = sol[gene_id];
                let variety = &VARIETIES[variety_id];
                week += variety.longevity;
            }
        }
    }
}

type HarvestPlan = [ [ HarvestableUnits; NUM_VARIETIES ]; SEASON_LENGTH ];

fn get_harvest_plan(sol: &Solution, varieties: &[Variety]) -> HarvestPlan {
    // build the harvest plan, which tells us how many units of each variety
    // are harvestable in each week
    let mut harvest_plan = [ [ 0; NUM_VARIETIES ]; SEASON_LENGTH ];
    for bed in 0..NUM_BEDS {
        let mut week: WeekId = 0;
        for crop in 0..MAX_CROPS_PER_BED {
            let gene_id = bed * MAX_CROPS_PER_BED + crop;
            let variety_id = sol[gene_id];
            let variety = &varieties[variety_id];

            for growth_week in 0..variety.longevity {
                let harvest_week = week+growth_week;
                let harvest_units = variety.growth_model[growth_week];
                if harvest_week < SEASON_LENGTH {
                    harvest_plan[harvest_week][variety_id] += harvest_units;
                }
            }

            week += variety.longevity;
        }
    }

    return harvest_plan;
}

fn get_score(sol: &Solution, varieties: &[Variety]) -> i32 {

    let harvest_plan = get_harvest_plan(sol, varieties);

    let mut score = 0;

    for g in sol.iter() {
        if *g != 0 {
            score -= 1;
        }
    }

    // score the harvest plan by determining how much of the harvestable
    // produce can be used to fill boxes
    for week in FIRST_BOX_WEEK..SEASON_LENGTH {
        let harvest = &harvest_plan[week];

        for variety_id in 0..NUM_VARIETIES {
            let variety = &varieties[variety_id];
            let harvestable_units = harvest[variety_id];

            score -= (harvestable_units - NUM_BOXES).abs();
        }
    }

    return score;
}

fn get_child<R: Rng + ?Sized, D: Distribution<usize>>(population: &Vec<Solution>, dist: &D, rng: &mut R) -> Solution {
    let mother = population[dist.sample(rng)];
    let father = population[dist.sample(rng)];
    let mut child = get_crossed(&mother, &father, rng);
    mutate(&mut child, rng);
    optimize(&mut child);
    return child;
}

fn print_solution(sol: &Solution) {
    for bed in 0..NUM_BEDS {
        for crop in 0..MAX_CROPS_PER_BED {
            let gene_id = bed * MAX_CROPS_PER_BED + crop;
            let variety_id = sol[gene_id];
            let variety = &VARIETIES[variety_id];
            print!("[{}] ", variety.name);
        }
        println!("");
    }
}

fn write_solution(sol: &Solution) {
    let path = Path::new("sol.csv");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create file: {}", why.description()),
        Ok(file) => file,
    };

    
    for bed in 0..NUM_BEDS {
        write!(file, "Bed{},", bed);
    }
    writeln!(file, "");

    for crop in 0..MAX_CROPS_PER_BED {
        for bed in 0..NUM_BEDS {
            let gene_id = bed * MAX_CROPS_PER_BED + crop;
            let variety_id = sol[gene_id];
            let variety = &VARIETIES[variety_id];
            write!(file, "{},", variety.name);
        }
        writeln!(file, "");
    }

    file.flush();
}

fn print_harvest_plan(harvest_plan: &HarvestPlan) {
    println!("");
    for v in 0..NUM_VARIETIES {
        print!("{}", VARIETIES[v].name);
        for w in 0..SEASON_LENGTH {
            print!("{:>6}", harvest_plan[w][v]);
        }
        println!("");
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let weights = 1..(POPULATION_SIZE+1);
    let dist = WeightedIndex::new(weights).unwrap();

    let mut population: Vec<_> = (0..POPULATION_SIZE)
        .map(|_| get_random(&mut rng))
        .collect();

    let mut best_score = -1000000;
    let mut gen = 0;

    loop {
        population.sort_by_cached_key(|p| get_score(p, &VARIETIES));

        let best_solution = population.last().expect("error").clone();
        let score = get_score(&best_solution, &VARIETIES);

        if score > best_score {
            best_score = score;

            let hp = get_harvest_plan(&best_solution, &VARIETIES);
            print_harvest_plan(&hp);
        
            println!("gen: {} score: {}", gen, best_score);

            if best_score > -6500 {
                write_solution(&best_solution);
                break;
            }
        }

        let next_generation: Vec<_> = (0..POPULATION_SIZE)
            .map(|_| get_child(&population, &dist, &mut rng))
            .collect();

        population.copy_from_slice(&next_generation);

        population[0] = best_solution;

        gen += 1;
    }
}