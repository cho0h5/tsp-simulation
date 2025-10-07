use crate::TSP;
use rand::prelude::*;

pub struct DFS<'a> {
    tsp: &'a TSP,
    best_path: Vec<usize>,
    best_cost: f32,
    path: Vec<usize>,
    visited: Vec<bool>,
}

impl<'a> DFS<'a> {
    pub fn new(tsp: &'a TSP) -> Self {
        let n = tsp.len();
        Self {
            tsp,
            best_path: Vec::new(),
            best_cost: f32::INFINITY,
            path: Vec::with_capacity(n + 1),
            visited: vec![false; n],
        }
    }

    pub fn search(&mut self) {
        let mut rng = rand::rng();
        let start = rng.random_range(0..self.tsp.len());

        self.path.push(start);
        self.visited[start] = true;

        self.dfs(start, 1, 0.0, start);

        println!("{:?}: {}", self.best_path, self.best_cost);
    }

    fn dfs(&mut self, curr: usize, depth: usize, g: f32, start: usize) {
        let n = self.tsp.len();

        if depth == n {
            let total = g + self.tsp.euclidean_distance(curr, start);
            if total < self.best_cost {
                self.best_cost = total;
                self.best_path = self.path.clone();
            }
            return;
        }

        for v in 0..n {
            if !self.visited[v] {
                self.visited[v] = true;
                self.path.push(v);

                let g2 = g + self.tsp.euclidean_distance(curr, v);
                self.dfs(v, depth + 1, g2, start);

                self.path.pop();
                self.visited[v] = false;
            }
        }
    }
}
