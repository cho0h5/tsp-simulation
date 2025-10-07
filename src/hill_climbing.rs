use crate::TSP;
use rand::prelude::*;

pub struct HillClimbing<'a> {
    tsp: &'a TSP,
    path: Vec<usize>,
    best_cost: f32,
}

impl<'a> HillClimbing<'a> {
    pub fn new(tsp: &'a TSP) -> Self {
        Self {
            tsp,
            path: Vec::new(),
            best_cost: f32::INFINITY,
        }
    }

    fn tour_len_of(&self, path: &[usize]) -> f32 {
        let n = path.len();
        let mut sum = 0.0f32;
        for k in 0..n - 1 {
            sum += self.tsp.euclidean_distance(path[k], path[k + 1]);
        }
        sum + self.tsp.euclidean_distance(path[n - 1], path[0])
    }

    fn random_init(&mut self) {
        let n = self.tsp.len();
        self.path = (0..n).collect();
        let mut rng = rand::rng();
        self.path.shuffle(&mut rng);
        self.best_cost = self.tour_len_of(&self.path);
    }

    pub fn search(&mut self) {
        self.random_init();
        let n = self.path.len();

        loop {
            let mut improved = false;
            let mut best_i = 0usize;
            let mut best_j = 0usize;
            let mut best_neighbor_cost = self.best_cost;

            for i in 0..n - 2 {
                for j in i + 2..n {
                    if i == 0 && j == n - 1 {
                        continue;
                    }

                    let mut cand = self.path.clone();
                    cand[i + 1..=j].reverse();

                    let cost = self.tour_len_of(&cand);
                    if cost < best_neighbor_cost {
                        best_neighbor_cost = cost;
                        best_i = i;
                        best_j = j;
                        improved = true;
                    }
                }
            }

            if improved {
                self.path[best_i + 1..=best_j].reverse();
                self.best_cost = best_neighbor_cost;
            } else {
                break;
            }
        }

        println!("{:?}: {}", self.path, self.best_cost);
    }
}
