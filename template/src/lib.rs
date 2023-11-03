use std::io::{BufRead, BufReader};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let solution = Self::default();
        for _line in reader.lines() {
            // Implement for problem
        }
        Ok(solution)
    }
}
