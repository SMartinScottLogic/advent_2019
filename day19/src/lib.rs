use std::io::{BufRead, BufReader};

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
                total += cpu.take_output().first().unwrap();
            }
        }
        // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

impl Solution {
    fn add_entry(&mut self, entry: i64) {
        self.entries.push(entry);
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
