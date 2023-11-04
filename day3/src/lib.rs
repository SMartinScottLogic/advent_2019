use std::{io::{BufRead, BufReader}, convert::Infallible, collections::{HashMap, HashSet}, cmp::{max, min}, iter};

use tracing::debug;

pub type ResultType = i32;

#[derive(Debug, Default)]
pub struct Solution {
    wires: Vec<Vec<Step>>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut occupation = HashSet::new();
        let mut crosses = HashSet::new();
        for wire in &self.wires {
            let o = Self::process_wire(wire);

            for pos in o {
                if !occupation.insert(pos) {
                    crosses.insert(pos);
                }
            }
        }
        let min_distance = crosses.iter().map(|(x, y)| x.abs()+y.abs()).filter(|d| *d!=0).min().unwrap();
        debug!(min_distance, crosses = debug(&crosses), "cross");
        // Implement for problem
        Ok(min_distance.try_into().unwrap())
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut occupation = HashMap::new();
        for wire in &self.wires {
            let o = Self::process_wire_v2(wire);
            let mut u = HashMap::new();
            for (id, v) in o.iter().enumerate() {
                u.entry(v).or_insert_with(Vec::new).push(1 + id);
            }
            for (pos, s) in u {
                let entry = occupation.entry(*pos).or_insert_with(Vec::new);
                entry.push(*s.iter().min().unwrap());
            }
        }
        let crosses = occupation.iter()
        .filter(|(_pos, e)| e.len() > 1)
        .map(|(pos, e)| {
            let mut total = 0;
            for d in e {
                total += d;
            }
            (pos, e, total)
        })
        .collect::<Vec<_>>();
        debug!(crosses = debug(&crosses), "crosses");

        let answer = crosses.iter().map(|(_pos, _e, total)| {
            total
        })
        .min().unwrap();
        // Implement for problem
        Ok((*answer).try_into().unwrap())
    }
}

impl Solution {
    fn add_wire(&mut self, wire: Vec<Step>) {
        self.wires.push(wire);
    }

    fn process_wire(wire: &Vec<Step>) -> HashSet<(isize, isize)> {
        let mut occupation = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        for step in wire {
            let (sx, sy) = match step {
                Step::Up(d) => (0_isize, *d),
                Step::Down(d) => (0_isize, -*d),
                Step::Left(d) => (-*d, 0_isize),
                Step::Right(d) => (*d, 0_isize),
            };
            if sx != 0 {
            for dx in min(0, sx)..=max(0, sx) {
                occupation.insert((x + dx, y));
            }
            }
            if sy != 0 {
            for dy in min(0, sy)..=max(0, sy) {
                occupation.insert((x, y + dy));
            }
            }
            x += sx;
            y += sy;
        }    
        debug!(occupation = debug(&occupation), "occ");
        occupation
    }

    fn process_wire_v2(wire: &Vec<Step>) -> Vec<(isize, isize)> {
        let mut occupation = Vec::new();
        let mut x = 0;
        let mut y = 0;
        for step in wire {
            let (sx, sy, dx, dy) = match step {
                Step::Up(d) => (0, *d, 0, 1),
                Step::Down(d) => (0, *d, 0, -1),
                Step::Left(d) => (*d, 0, -1, 0),
                Step::Right(d) => (*d, 0, 1, 0),
            };
            if dx!=0 {
                for ex in 1..=sx {
                    x += dx;
                    occupation.push((x, y));
                }
            }
            if dy!=0 {
                for ey in 1..=sy {
                    y += dy;
                    occupation.push((x, y));
                }
            }
        }    
        debug!(occupation = debug(&occupation), "occ");
        occupation
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().flatten() {
            let wire = line.split(',')
            .map(|s| s.into())
            .collect();
            solution.add_wire(wire);
        }
        Ok(solution)
    }
}

#[derive(Debug)]
enum Step {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        let distance = &value[1..].parse().unwrap();
        match value.chars().nth(0).unwrap() {
            'U' => Self::Up(*distance),
            'D' => Self::Down(*distance),
            'L' => Self::Left(*distance),
            'R' => Self::Right(*distance),
            _ => unreachable!()
        }
    }
}
