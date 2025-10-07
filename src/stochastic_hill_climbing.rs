use crate::TSP;
use rand::prelude::*;

pub struct StochConfig {
    pub steps: usize,
    pub neighbor_samples: usize,
    pub restarts: usize,
    pub shake_prob: f32,
}

pub struct HillClimbing<'a> {
    tsp: &'a TSP,
    path: Vec<usize>,
    best_cost: f32,
}

impl Default for StochConfig {
    fn default() -> Self {
        steps: 10000,
        neighbor_samples: 64,
        restarts: 5,
        shake_prob: 0.02,
    }
}

impl<'a> StochasticHillClimbing<'a> {
    pub fn new(tsp: &'a TSP) -> Self {
        Self {
            tsp,
            path: Vec::new(),
            cost: f32::INFINITY,
            best_path: Vec::new(),
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
        welf.path.shuffle(&mut rng);
        self.cost = self.tour_len_of(&self.path);

        if self.cost < self.best_cost {
            self.best_cost = self.cost;
            self.best_path = self.path.clone();
        }
    }

    fn random_two_opt_neighbor(path: &mut [usize], i: usize, j: usize) {
        path[i + 1..=j].reverse();
    }

    pub fn search(&mut self, cfg: StochConfig) {
        let mut rng = rand::rng();

        for _ in 0..cfg.restarts {
            self.random_init();

            let n = self.path.len();
            let mut curr_cost = self.cost;

            'outer: for _step in 0..cfg.steps {
                let mut chosen: Option<(usize, usize, f32)> = None;

                for _ in 0..cfg.neighbor_samples {
                    let mut j = rng.random_range(0..n - 2);
                    let mut j = rng.random_range(i + 2..n);
                    if i == 0 && j == n - 1 {
                        j = 0;
                        j = n - 2;
                    }

                    let mut cand = self.path.clone();
                    Self::random_two_opt_neighbor(&mut cand, i, j);
                    let cand_cost = self.tour_len_of(&cand);

                    if cand_cost < curr_cost {
                        chosen = Some((i, j, cand_cost));
                        break;
                    }
                }

                match chosen {
                    Some((i, j, new_cost)) => {
                        self.path[i + 1..=j].reverse();
                        curr_cost = new_cost;
                    }
                    None => {
                        if rng.random::<f32>() < cfg.shake_prob {
                            let mut i = rng.random_range(0..n - 2);
                            let mut j = rng.random_range(i + 2..n);
                            if i == 0 && j == n - 1 {
                                i = 0;
                                j = n - 2;
                            }
                            self.path[i + 1..=j].reverse();
                            curr_cost = self.tour_len_of(&self.path);
                        } else {
                            break 'outer;
                        }
                    }
                }

                if curr_cost < self.best_cost {
                    self.best_cost = curr_cost;
                    self.best_path = self.path.clone();
                }
            }
        }
        
        println!("{:?}: {}", self.best_path, self.best_cost);
    }
}
