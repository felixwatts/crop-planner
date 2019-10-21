mod constant;
mod harvest_plan;
mod rand;
mod solution;
mod solver;
mod variety;

fn main() {
    let varieties = crate::variety::new();
    let mut solver = crate::solver::Solver::new(&varieties);

    let mut best_score = -1000000;
    let mut gen = 0;

    loop {
        solver.step();

        let score = solver.get_best_score();

        if score > best_score {
            best_score = score;

            let best_solution = solver.get_best_solution();

            let harvest_plan = crate::solution::to_harvest_plan(&best_solution, &varieties);
            crate::harvest_plan::print_harvest_plan(&harvest_plan, &varieties);
        
            println!("gen: {} score: {}", gen, best_score);
        }

        gen += 1;
    }
}
