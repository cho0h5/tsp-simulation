use rand::prelude::*;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct TSP {
    points: Vec<Point>,
}

impl TSP {
    pub fn new(num_points: usize, min: i32, max: i32) -> Self {
        let mut rng = rand::rng();

        let points = (0..num_points)
            .map(|_| Point {
                x: rng.random_range(min..max),
                y: rng.random_range(min..max),
            })
            .collect();

        Self { points }
    }

    pub fn euclidean_distance(&self, i: usize, j: usize) -> f32 {
        ((self.points[j].x as f32 - self.points[i].x as f32).powf(2.0)
            + (self.points[j].y as f32 - self.points[i].y as f32).powf(2.0))
        .sqrt()
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}
