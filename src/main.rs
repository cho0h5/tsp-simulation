use rand::prelude::*;
use std::collections::HashSet;
use std::time::{Duration, Instant};

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
        ((self.points[j].x as f32 - self.points[i].x as f32).powf(2.0)
            + (self.points[j].y as f32 - self.points[i].y as f32).powf(2.0))
        .sqrt()
    }
}

struct Greedy<'a> {
    tsp: &'a TSP,
    path: Vec<usize>,
}

impl<'a> Greedy<'a> {
    fn new(tsp: &'a TSP) -> Self {
        Self {
            tsp,
            path: Vec::new(),
        }
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

        open_len
            + self
                .tsp
                .euclidean_distance(*self.path.last().unwrap(), *self.path.first().unwrap())
    }
}

struct DFS<'a> {
    tsp: &'a TSP,
    best_path: Vec<usize>,
    best_cost: f32,
    path: Vec<usize>,
    visited: Vec<bool>,
}

impl<'a> DFS<'a> {
    fn new(tsp: &'a TSP) -> Self {
        let n = tsp.points.len();
        Self {
            tsp,
            best_path: Vec::new(),
            best_cost: f32::INFINITY,
            path: Vec::with_capacity(n + 1),
            visited: vec![false; n],
        }
    }

    fn search(&mut self) {
        let mut rng = rand::rng();
        let start = rng.random_range(0..self.tsp.points.len());

        self.path.push(start);
        self.visited[start] = true;

        self.dfs(start, 1, 0.0, start);

        println!("{:?}: {}", self.best_path, self.best_cost);
    }

    fn dfs(&mut self, curr: usize, depth: usize, g: f32, start: usize) {
        let n = self.tsp.points.len();

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

struct BranchAndBound<'a> {
    tsp: &'a TSP,
    best_path: Vec<usize>,
    best_cost: f32,
    path: Vec<usize>,
    visited: Vec<bool>,
    min_outgoing: Vec<f32>,
}

impl<'a> BranchAndBound<'a> {
    fn new(tsp: &'a TSP) -> Self {
        let n = tsp.points.len();

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

    fn search(&mut self) {
        let mut rng = rand::rng();
        let start = rng.random_range(0..self.tsp.points.len());

        self.path.push(start);
        self.visited[start] = true;

        self.dfs(start, 1, 0.0, start);

        println!("{:?}: {}", self.best_path, self.best_cost);
    }

    fn dfs(&mut self, curr: usize, depth: usize, g: f32, start: usize) {
        let n = self.tsp.points.len();

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

struct HillClimbing<'a> {
    tsp: &'a TSP,
    path: Vec<usize>,
    best_cost: f32,
}

impl<'a> HillClimbing<'a> {
    fn new(tsp: &'a TSP) -> Self {
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
        let n = self.tsp.points.len();
        self.path = (0..n).collect();
        let mut rng = rand::rng();
        self.path.shuffle(&mut rng);
        self.best_cost = self.tour_len_of(&self.path);
    }

    fn search(&mut self) {
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
