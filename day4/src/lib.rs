use std::io::{BufRead, BufReader};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    from: u64,
    to: u64,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut count = 0;
        for n in self.from..=self.to {
            if Self::is_valid_step1(n) {
                count += 1;
            }
        }
        // Implement for problem
        Ok(count)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut count = 0;
        for n in self.from..=self.to {
            if Self::is_valid_step2(n) {
                count += 1;
            }
        }
        // Implement for problem
        Ok(count)
    }
}

impl Solution {
    fn set_range(&mut self, from: u64, to: u64) {
        self.from = from;
        self.to = to;
    }

    fn is_valid_step1(value: u64) -> bool {
        let digits = format!("{value}").chars().collect::<Vec<_>>();
        // never decreasing
        let mut last_digit = None;
        for digit in &digits {
            if matches!(last_digit, Some(v) if v > digit) {
                return false;
            }
            last_digit = Some(digit);
        }
        // Two identical
        let mut has_dupe = false;
        let mut last_digit = None;
        for digit in &digits {
            if matches!(last_digit, Some(v) if v == digit) {
                has_dupe = true;
            }
            last_digit = Some(digit);
        }
        if !has_dupe {
            return false;
        }
        true
    }

    fn is_valid_step2(value: u64) -> bool {
        let digits = format!("{value}").chars().collect::<Vec<_>>();
        // never decreasing
        let mut last_digit = None;
        for digit in &digits {
            if matches!(last_digit, Some(v) if v > digit) {
                return false;
            }
            last_digit = Some(digit);
        }
        // Two identical
        let mut count = 0;
        let mut contains_double = false;
        let mut last_digit = None;
        for digit in &digits {
            match last_digit {
                None => {
                    last_digit = Some(digit);
                    count = 1;
                }
                Some(v) if v == digit => {
                    count += 1;
                }
                Some(_) => {
                    if count == 2 {
                        contains_double = true;
                    }
                    last_digit = Some(digit);
                    count = 1;
                }
            }
        }
        if count == 2 {
            contains_double = true;
        }
        if !contains_double {
            return false;
        }
        true
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().flatten() {
            let (from, to) = line.split_once('-').unwrap();
            let from = from.parse().unwrap();
            let to = to.parse().unwrap();
            solution.set_range(from, to);
        }
        Ok(solution)
    }
}
