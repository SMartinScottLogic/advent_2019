use std::io::{BufRead, BufReader};

use tracing::debug;

pub type ResultType = i64;

mod intcode;

#[derive(Debug, Default)]
pub struct Solution {
    entries: Vec<i64>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, is_full: bool) -> Self::Result {
        let mut cpu = intcode::Cpu::new(0, &self.entries);
        cpu.input(1);
        while !cpu.has_halted() {
            cpu.execute();
        }
        debug!(cpu = debug(&cpu), "inspect");
        let outputs = cpu.take_output();
        if is_full {
            assert_eq!(1, outputs.len());
        }
        // Implement for problem
        Ok(*outputs.first().unwrap())
    }

    fn answer_part2(&self, is_full: bool) -> Self::Result {
        let mut cpu = intcode::Cpu::new(0, &self.entries);
        cpu.input(2);
        while !cpu.has_halted() {
            cpu.execute();
        }
        debug!(cpu = debug(&cpu), "inspect");
        let outputs = cpu.take_output();
        if is_full {
            assert_eq!(1, outputs.len());
        }
        // Implement for problem
        Ok(*outputs.first().unwrap())
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
