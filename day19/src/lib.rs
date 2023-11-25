use std::io::{BufRead, BufReader};

use tracing::{info, debug};
use utils::Matrix;

mod intcode;

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    entries: Vec<i64>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut total = 0;
        for y in 0..50 {
            for x in 0..50 {
                let mut cpu = intcode::Cpu::new(0, &self.entries);
                loop {
                    cpu.execute();
                    if cpu.needs_input() {
                        cpu.input(x);
                        cpu.input(y);
                    }
                    if cpu.has_halted() {
                        break;
                    }
                }
                let score = cpu.take_output();
                let score = *score.first().unwrap();
                total += score;
            }
        }

        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut display = Matrix::new();
        let mut total = 0;
        for y in 0..1500 {
            let start = 0;
            let end = 1500;
            for x in start..=end {
                if x < 0 {
                    continue;
                }
                let mut cpu = intcode::Cpu::new(0, &self.entries);
                loop {
                    cpu.execute();
                    if cpu.needs_input() {
                        cpu.input(x as i64);
                        cpu.input(y as i64);
                    }
                    if cpu.has_halted() {
                        break;
                    }
                }
                let score = cpu.take_output();
                let score = *score.first().unwrap();
                display.set(x, y, score);
                total += score;
            }
        }
        let r = display.sparse_iter().filter(|(_, v)| *v==&1).filter_map(|((x, y), v)|
            if Self::check_fits(&display, *x, *y, 100, 100) {
                debug!(r = x * 10_000 + y, x, y, "fits");
                Some(x * 10_000 + y)
            } else {
                None
            }
        ).min();
        info!(r, "r");
         // Implement for problem
        Ok(r.unwrap() as ResultType)
    }
}

impl Solution {
    fn add_entry(&mut self, entry: i64) {
        self.entries.push(entry);
    }

    fn check_fits(
        pull_locations: &Matrix<i64>,
        point_x: isize,
        point_y: isize,
        size_w: isize,
        size_h: isize,
    ) -> bool {
        // check point, right top corner and bottom left corner
        // because of the shape of the tractor beam, if these 3 points
        // are pull locations, then we have a square of size_w * size_h
        if let Some(1) = pull_locations.get(point_x, point_y) {
            if let Some(1) = pull_locations.get(point_x + size_w - 1, point_y) {
                if let Some(1) = pull_locations.get(point_x, point_y + size_h - 1) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }else {
            false
        }
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().flatten() {
            for e in line.split(',') {
                if let Ok(e) = e.parse() {
                    solution.add_entry(e);
                }
            }
        }
        Ok(solution)
    }
}
