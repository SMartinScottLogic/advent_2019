use std::io::{BufRead, BufReader};

use tracing::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    entries: Vec<u64>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut ip = 0;
        let entries = &mut self.entries.clone()[..];
        entries[1] = 12;
        entries[2] = 2;
        loop {
            if Self::run_command(ip, entries) {
                break;
            }
            ip += 4;
        }
        // Implement for problem
        Ok(entries[0])
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

impl Solution {
    fn add_entry(&mut self, value: u64) {
        self.entries.push(value);
    }

    fn run_command(ip: usize, entries: &mut [u64]) -> bool {
        let op = entries[ip];
        debug!(ip, op, "run_command");
        match op {
            99 => true,
            1 => {
                let addr_a = entries[ip + 1] as usize;
                let addr_b = entries[ip + 2] as usize;
                let addr_r = entries[ip + 3] as usize;
                let r = entries[addr_a] + entries[addr_b];
                debug!(addr_a, addr_b, addr_r, r, "add");
                entries[addr_r] = r;
                false
            }
            2 => {
                let addr_a = entries[ip + 1] as usize;
                let addr_b = entries[ip + 2] as usize;
                let addr_r = entries[ip + 3] as usize;
                let r = entries[addr_a] * entries[addr_b];
                debug!(addr_a, addr_b, addr_r, r, "mult");
                entries[addr_r] = r;
                false
            }
            _ => unreachable!(),
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
