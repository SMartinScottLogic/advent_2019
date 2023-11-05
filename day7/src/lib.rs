use std::{
    collections::HashMap,
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
        let mut max_output = 0;
        for a in 5..=9 {
            for b in 5..=9 {
                if a != b {
                    for c in 5..=9 {
                        if a != c && b != c {
                            for d in 5..=9 {
                                if a != d && b != d && c != d {
                                    for e in 5..=9 {
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
}

impl Solution {
    fn add_entry(&mut self, value: i64) {
        self.entries.push(value);
    }

    fn run_trial(&self, phases: &[i64]) -> i64 {
        let mut thruster_outputs = Vec::new();
        let mut output = 0;
        let mut cpus = phases
            .iter()
            .enumerate()
            .map(|(id, phase)| {
                let mut cpu = intcode::CPU::new(id, &self.entries);
                cpu.input(*phase);
                cpu
            })
            .collect::<Vec<_>>();
        if let Some(cpu) = cpus.get_mut(0) {
            cpu.input(0);
        }
        loop {
            for cpu in cpus.iter_mut() {
                cpu.execute();
            }
            // output -> next in chain
            let mut new_inputs = HashMap::new();
            for cpu in cpus.iter_mut() {
                let outputs = cpu.take_output();
                if !outputs.is_empty() {
                    if cpu.id() == 4 {
                        thruster_outputs.append(&mut outputs.clone());
                    }
                    new_inputs.insert((cpu.id() + 1) % phases.len(), outputs);
                }
            }
            if !new_inputs.is_empty() {
                debug!(new_inputs = debug(&new_inputs), "new inputs");
                for (id, inputs) in new_inputs {
                    let cpu = cpus.get_mut(id).unwrap();
                    for input in inputs {
                        cpu.input(input);
                    }
                }
            }
            if cpus.iter().all(|cpu| cpu.has_halted()) {
                break;
            }
        }
        debug!(
            phases = debug(phases),
            output,
            thruster_outputs = debug(&thruster_outputs),
            "trial"
        );
        output = *thruster_outputs.iter().next_back().unwrap_or(&-1);
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
