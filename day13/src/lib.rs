use std::io::{BufRead, BufReader};

use tracing::debug;
use utils::Matrix;

pub type ResultType = u64;

mod intcode;

#[derive(Debug, Default)]
pub struct Solution {
    entries: Vec<i64>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut cpu = intcode::Cpu::new(0, &self.entries);
        let mut tiles = Matrix::new();
        while !cpu.has_halted() {
            cpu.execute();
            if cpu.needs_input() {
                panic!()
            }
            if cpu.output().len() == 3 {
                if let [x, y, b] = cpu.take_output()[..] {
                    tiles.set(x as isize, y as isize, b);
                } else {
                    panic!();
                }
            }
        }
        let a = tiles.sparse_iter().filter(|(_, b)| *b == &2).count();
        // Implement for problem
        Ok(a.try_into().unwrap())
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut entries = self.entries.clone();
        entries[0] = 2;
        let mut cpu = intcode::Cpu::new(0, &entries);
        let mut tiles = Matrix::new();
        let mut score = 0;
        while !cpu.has_halted() {
            cpu.execute();
            if cpu.needs_input() {
                let px = match tiles.sparse_iter().find(|(_, b)| *b == &3) {
                    None => panic!("No paddle on the board"),
                    Some(((x, _), _)) => x,
                };
                let bx = match tiles.sparse_iter().find(|(_, b)| *b == &4) {
                    None => panic!("No ball on the board"),
                    Some(((x, _), _)) => x,
                };

                debug!(bx, px, delta = (bx - px).signum(), "compute move");
                cpu.input((bx - px).signum() as i64);
            }
            if cpu.output().len() == 3 {
                if let [x, y, b] = cpu.take_output()[..] {
                    if x == -1 && y == 0 {
                        score = b;
                    }
                    tiles.set(x as isize, y as isize, b);
                } else {
                    panic!();
                }
            }
        }
        // Implement for problem
        Ok(score.try_into().unwrap())
    }
}

impl Solution {
    fn add_entry(&mut self, value: i64) {
        self.entries.push(value);
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
