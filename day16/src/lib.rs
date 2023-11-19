use std::io::{BufRead, BufReader};

use tracing::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    input: String,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut elements = self
            .input
            .chars()
            .map(|v| v.to_digit(10).unwrap() as i64)
            .collect::<Vec<_>>();
        for pass in 0..100 {
            let mut next_elements = elements.clone();
            let num_chars = elements.len();
            for output_digit in 0..num_chars {
                let row_num = output_digit + 1;
                let pattern = [0, 1, 0, -1].repeat(num_chars);
                let pattern = pattern
                    .iter()
                    .flat_map(|v| std::iter::repeat(*v).take(row_num))
                    .skip(1)
                    .take(num_chars)
                    .collect::<Vec<_>>();
                debug!(output_digit, pattern = debug(&pattern), "pattern");
                let c: i64 = pattern
                    .iter()
                    .zip(elements.iter())
                    .map(|(a, b)| a * b)
                    .sum::<i64>()
                    .abs()
                    % 10;
                debug!(c = debug(&c), "zip");
                next_elements[output_digit] = c;
            }
            debug!(pass, next_elements = debug(&next_elements), "next");
            elements = next_elements;
        }
        let result = elements.iter().take(8).fold(0, |mut acc, v| {
            acc *= 10;
            acc += v;
            acc
        });
        // Implement for problem
        Ok(result.try_into().unwrap())
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

impl Solution {
    fn set_input(&mut self, input: String) {
        self.input = input;
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().flatten() {
            solution.set_input(line);
        }
        Ok(solution)
    }
}
