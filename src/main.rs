use rand::prelude::*;
use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct TSP {
    points: Vec<Point>,
}

impl TSP {
    fn new(num_points: usize, min: i32, max: i32) -> Self {
        let mut rng = rand::rng();

        let points = (0..num_points)
            .map(|_| Point {
                x: rng.random_range(min..max),
                y: rng.random_range(min..max),
            })
        .collect();

        Self { points }
    }

    fn euclidean_distance(&self, i: usize, j: usize) -> f32 {
        ((self.points[j].x as f32 - self.points[i].x as f32).powf(2.0) +
         (self.points[j].y as f32 - self.points[i].y as f32).powf(2.0)).sqrt()
    }
}

struct Greedy<'a> {
    tsp: &'a TSP,
    path: Vec<usize>,
}

impl<'a> Greedy<'a> {
    fn new(tsp: &'a TSP) -> Self {
        Self { tsp, path: Vec::new() }
    }

    fn search(&mut self) {
        let mut rng = rand::rng();
        let start = rng.random_range(0..self.tsp.points.len());
        self.path.push(start);

        let mut visited: HashSet<usize> = HashSet::new();
        visited.insert(start);

        let mut current = start;
        while visited.len() < self.tsp.points.len() {
            let mut min_next: Option<usize> = None;
            let mut min_distance: f32 = f32::INFINITY;

            for candidate in 0..self.tsp.points.len() {
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

        open_len + self.tsp.euclidean_distance(*self.path.last().unwrap(), *self.path.first().unwrap())
    }
}

fn main() {
    let tsp = TSP::new(5, 0, 100);
    let mut greedy = Greedy::new(&tsp);
    greedy.search();
}
