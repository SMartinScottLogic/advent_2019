use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
};

use tracing::debug;

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
        let mut ip = 0;
        let mem = &mut self.entries.clone()[..];
        let mut outputs = Vec::new();
        let mut inputs = VecDeque::new();
        inputs.push_back(1);
        loop {
            let (term, delta) = intcode::execute(ip, mem, &mut inputs, &mut outputs);
            if term {
                break;
            }
            ip = delta;
        }
        debug!(mem = debug(mem), output = debug(&outputs), "done");

        let output = outputs.iter().next_back().unwrap_or(&-1);

        // Implement for problem
        Ok(*output)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut ip = 0;
        let mem = &mut self.entries.clone()[..];
        let mut outputs = Vec::new();
        let mut inputs = VecDeque::new();
        inputs.push_back(5);
        loop {
            let (term, delta) = intcode::execute(ip, mem, &mut inputs, &mut outputs);
            if term {
                break;
            }
            ip = delta;
            debug!(mem = debug(&mem), ip, "trace");
        }
        debug!(mem = debug(mem), output = debug(&outputs), "done");

        let output = outputs.iter().next_back().unwrap_or(&-1);

        // Implement for problem
        Ok(*output)
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
