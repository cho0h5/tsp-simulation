use crate::TSP;
use rand::prelude::*;

pub struct BranchAndBound<'a> {
    tsp: &'a TSP,
    best_path: Vec<usize>,
    best_cost: f32,
    path: Vec<usize>,
    visited: Vec<bool>,
    min_outgoing: Vec<f32>,
}

impl<'a> BranchAndBound<'a> {
    pub fn new(tsp: &'a TSP) -> Self {
        let n = tsp.len();

        let mut min_outgoing = vec![0.0f32; n];
        for i in 0..n {
            let mut m = f32::INFINITY;
            for j in 0..n {
                if i == j {
                    continue;
                }
                m = m.min(tsp.euclidean_distance(i, j));
            }
            min_outgoing[i] = if m.is_finite() { m } else { 0.0 };
        }

        Self {
            tsp,
            best_path: Vec::new(),
            best_cost: f32::INFINITY,
            path: Vec::with_capacity(n + 1),
            visited: vec![false; n],
            min_outgoing,
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

        let mut optimistic = 0.0f32;
        for i in 0..n {
            if !self.visited[i] {
                optimistic += self.min_outgoing[i];
            }
        }
        if g + optimistic >= self.best_cost {
            return;
        }

        if depth == n {
            let total = g + self.tsp.euclidean_distance(curr, start);
            if total < self.best_cost {
                self.best_cost = total;
                self.best_path = self.path.clone();
            }
            return;
        }

        let mut candidates: Vec<(usize, f32)> = Vec::new();
        for v in 0..n {
            if !self.visited[v] {
                candidates.push((v, self.tsp.euclidean_distance(curr, v)));
            }
        }
        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for (v, w) in candidates {
            if g + w >= self.best_cost {
                continue;
            }

            self.visited[v] = true;
            self.path.push(v);

            self.dfs(v, depth + 1, g + w, start);

            self.path.pop();
            self.visited[v] = false;
        }
    }
}
