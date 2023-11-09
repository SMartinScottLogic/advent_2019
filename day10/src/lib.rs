use std::{
    cmp::max,
    fmt::Debug,
    io::{BufRead, BufReader},
};

use tracing::{debug, info};
use utils::Matrix;

pub type ResultType = i64;

#[derive(Default)]
pub struct Solution {
    asteroids: Matrix,
}
impl Debug for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Solution")
            //.field("asteroids", &self.asteroids.len())
            .finish()
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let (maxx, maxy) = self.asteroids.dimensions();
        let mut max_count = 0;
        let mut best_x = 0;
        let mut best_y = 0;
        for y in 0..=maxy {
            for x in 0..=maxx {
                if let Some(1) = self.asteroids.get(x, y) {
                    let count = self.count_visible(x, y);
                    if count > max_count {
                        max_count = count;
                        best_x = x;
                        best_y = y;
                    }
                    //break 'end;
                }
            }
        }
        info!(best_x, best_y, max_count, "answer");
        // Implement for problem
        Ok(max_count)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

impl Solution {
    fn add_asteroid(&mut self, x: usize, y: usize) {
        self.asteroids
            .set(x.try_into().unwrap(), y.try_into().unwrap(), 1);
    }

    fn count_visible(&self, x: isize, y: isize) -> i64 {
        let (max_x, max_y) = self.asteroids.dimensions();
        let max_delta = max(max(x, max_x - x), max(y, max_y - y));
        debug!(x, y, max_x, max_y, max_delta, "delta");
        let mut status = Matrix::new();
        for delta in 1..=max_delta {
            for dy in -delta..=delta {
                for dx in -delta..=delta {
                    if dy.abs() != delta && dx.abs() != delta {
                        continue;
                    }
                    let px = x + dx;
                    let py = y + dy;
                    if px < 0 || px > max_x {
                        continue;
                    }
                    if py < 0 || py > max_y {
                        continue;
                    }
                    if let Some(1) = self.asteroids.get(px, py) {
                        if status.get(px, py).is_none() {
                            debug!(x, y, px, py, delta, max_delta, max_x, max_y, "sight");
                            status.set(px, py, 1);
                            self.mark_unseeable(x, y, max_x, max_y, dx, dy, &mut status, 0);
                        }
                    }
                }
            }
        }
        status.set(x, y, 0);
        //status.display();

        let mut count = 0;
        for y in 0..=max_y {
            for x in 0..=max_x {
                count += match status.get(x, y) {
                    None => 0,
                    Some(v) => *v,
                }
            }
        }
        count
    }

    fn mark_unseeable(
        &self,
        x: isize,
        y: isize,
        max_x: isize,
        max_y: isize,
        dx: isize,
        dy: isize,
        status: &mut Matrix,
        value: i64,
    ) {
        let gcd = greatest_common_divisor(dx.abs(), dy.abs());
        let step_x = dx / gcd;
        let step_y = dy / gcd;
        let mut px = x + dx;
        let mut py = y + dy;
        loop {
            px += step_x;
            py += step_y;
            if px < 0 || px > max_x {
                break;
            }
            if py < 0 || py > max_y {
                break;
            }
            if let Some(1) = self.asteroids.get(px, py) {
                if status.get(px, py).is_none() {
                    status.set(px, py, value);
                }
            }
            debug!(x, y, max_x, max_y, px, py, gcd, "probe");
        }
    }
}

fn greatest_common_divisor(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();

        for (y, line) in reader.lines().flatten().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    solution.add_asteroid(x, y);
                }
            }
            // Implement for problem
        }
        Ok(solution)
    }
}
