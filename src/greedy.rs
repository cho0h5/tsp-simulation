use crate::TSP;
use rand::prelude::*;
use std::collections::HashSet;

pub struct Greedy<'a> {
    tsp: &'a TSP,
    path: Vec<usize>,
}

impl<'a> Greedy<'a> {
    pub fn new(tsp: &'a TSP) -> Self {
        Self {
            tsp,
            path: Vec::new(),
        }
    }

    pub fn search(&mut self) {
        let mut rng = rand::rng();
        let start = rng.random_range(0..self.tsp.len());
        self.path.push(start);

        let mut visited: HashSet<usize> = HashSet::new();
        visited.insert(start);

        let mut current = start;
        while visited.len() < self.tsp.len() {
            let mut min_next: Option<usize> = None;
            let mut min_distance: f32 = f32::INFINITY;

            for candidate in 0..self.tsp.len() {
                if visited.contains(&candidate) {
                    continue;
                }

                let d = self.tsp.euclidean_distance(current, candidate);
                if d < min_distance {
                    min_distance = d;
                    min_next = Some(candidate);
                }
            }

            let next = min_next.unwrap();
            self.path.push(next);
            visited.insert(next);
            current = next;
        }

        let tour_len = self.get_tour_len();
        println!("{:?}: {}", self.path, tour_len);
    }

    fn get_tour_len(&self) -> f32 {
        let mut open_len = 0.0f32;
        for w in self.path.windows(2) {
            open_len += self.tsp.euclidean_distance(w[0], w[1]);
        }

        open_len
            + self
                .tsp
                .euclidean_distance(*self.path.last().unwrap(), *self.path.first().unwrap())
    }
}
