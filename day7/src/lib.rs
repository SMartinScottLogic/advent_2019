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
        let mut max_output = 0;
        for a in 0..=4 {
            for b in 0..=4 {
                if a != b {
                    for c in 0..=4 {
                        if a != c && b != c {
                            for d in 0..=4 {
                                if a != d && b != d && c != d {
                                    for e in 0..=4 {
                                        if a != e && b != e && c != e && d != e {
                                            let phases = vec![a, b, c, d, e];
                                            let output = self.run_trial(&phases);
                                            if output > max_output {
                                                max_output = output;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // Implement for problem
        Ok(max_output)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

impl Solution {
    fn add_entry(&mut self, value: i64) {
        self.entries.push(value);
    }

    fn run_trial(&self, phases: &[i64]) -> i64 {
        let mut output = 0;
        for phase in phases {
            let mut ip = 0;
            let mem = &mut self.entries.clone()[..];
            let mut outputs = Vec::new();
            let mut inputs = VecDeque::new();
            inputs.push_back(*phase);
            inputs.push_back(output);
            loop {
                let (term, delta) = intcode::execute(ip, mem, &mut inputs, &mut outputs);
                if term {
                    break;
                }
                ip = delta;
            }
            debug!(mem = debug(mem), output = debug(&outputs), "done");

            output = *outputs.iter().next_back().unwrap_or(&-1);
        }
        debug!(phases = debug(phases), output, "trial");
        output
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
