mod branch_and_bound;
mod dfs;
mod greedy;
mod hill_climbing;
mod tsp;

use branch_and_bound::BranchAndBound;
use dfs::DFS;
use greedy::Greedy;
use hill_climbing::HillClimbing;
use std::time::{Duration, Instant};
use tsp::TSP;

fn measure_time<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let start = Instant::now();
    let out = f();
    let dur = start.elapsed();
    (out, dur)
}

fn main() {
    let tsp = TSP::new(100, 0, 100);

    let (_, dur) = measure_time(|| {
        let mut greedy = Greedy::new(&tsp);
        greedy.search();
    });
    println!("Greedy: {:?}", dur);

    // let (_, dur) = measure_time(|| {
    //     let mut dfs = DFS::new(&tsp);
    //     dfs.search();
    // });
    // println!("DFS: {:?}", dur);

    // let (_, dur) = measure_time(|| {
    //     let mut branch_and_bound = BranchAndBound::new(&tsp);
    //     branch_and_bound.search();
    // });
    // println!("B&B: {:?}", dur);

    let (_, dur) = measure_time(|| {
        let mut hill_climbing = HillClimbing::new(&tsp);
        hill_climbing.search();
    });
    println!("HillClimbing: {:?}", dur);
}
