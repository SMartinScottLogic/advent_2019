use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use tracing::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    image: String,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let pane_counts = self
            .image
            .chars()
            .enumerate()
            //.take(310)
            .map(|(id, v)| (id, id / (25 * 6), v))
            .fold(HashMap::new(), |mut acc, (_pixel_id, pane, value)| {
                let entry: &mut HashMap<char, u64> = acc.entry(pane).or_default();
                *entry.entry(value).or_insert(0) += 1;
                acc
            });
        debug!(pane_counts = debug(&pane_counts), "count");
        let (_pane_id, min_counts) = pane_counts
            .iter()
            .min_by_key(|(_pane_id, counts)| counts.get(&'0').unwrap_or(&0))
            .unwrap();
        debug!(min = debug(min_counts), "min");
        let answer = min_counts.get(&'1').unwrap_or(&0) * min_counts.get(&'2').unwrap_or(&0);
        // Implement for problem
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let pic = self
            .image
            .chars()
            .enumerate()
            .map(|(id, v)| {
                let pane_id = id / (25 * 6);
                let offset = id % (25 * 6);
                let x = offset % 25;
                let y = offset / 25;
                (pane_id, x, y, v.to_digit(10).unwrap())
            })
            .fold(HashMap::new(), |mut acc, (_pane_id, x, y, value)| {
                let entry = acc.entry((x, y)).or_insert(2);
                if *entry == 2 {
                    *entry = value;
                }
                acc
            });
        for y in 0..=6 {
            let mut s = String::new();
            for x in 0..=25 {
                let c = match pic.get(&(x, y)) {
                    Some(0) => ' ',
                    Some(1) => '█',
                    _ => '.',
                };
                s.push(c);
            }
            println!("{s}");
        }
        // Implement for problem
        Ok(0)
    }
}

impl Solution {
    fn set_image(&mut self, image: String) {
        self.image = image;
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().flatten() {
            solution.set_image(line);
            // Implement for problem
        }
        Ok(solution)
    }
}
