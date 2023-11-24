use pathfinding::prelude::astar;
use std::{
    cmp::max,
    collections::HashMap,
    io::{BufRead, BufReader},
};

use tracing::info;
use utils::Point;

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    cells: HashMap<Point, char>,
    maxx: i64,
    maxy: i64,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Find start
        let (position, ..) = self.cells.iter().find(|(_, c)| *c == &'@').unwrap();
        let (path, len) = astar(
            &(position.to_owned(), Self::remaining_keys(&self.cells)),
            |p| self.successors(p),
            |p| self.heuristic(p),
            |p| self.success_end(p),
        )
        .unwrap();
        info!(len, path = debug(path), "result");
        // Implement for problem
        Ok(len)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

impl Solution {
    fn set_cell(&mut self, point: Point, c: char) {
        self.cells.insert(point, c);
        self.maxx = max(self.maxx, point.x());
        self.maxy = max(self.maxy, point.y());
    }

    fn remaining_keys(cells: &HashMap<Point, char>) -> Vec<char> {
        cells
            .iter()
            .filter(|(_, c)| (**c).is_lowercase())
            .map(|(_, v)| *v)
            .collect()
    }

    fn successors(
        &self,
        (position, remaining_keys): &(Point, Vec<char>),
    ) -> Vec<((Point, Vec<char>), i64)> {
        position
            .cardinal()
            .iter()
            .filter_map(|neigh| {
                let v = self.cells.get(neigh).unwrap_or(&'#');
                match v {
                    '#' => None,
                    '.' => Some(((*neigh, remaining_keys.clone()), 1)),
                    '@' => Some(((*neigh, remaining_keys.clone()), 1)),
                    c if c.is_uppercase()
                        && remaining_keys.contains(&c.to_lowercase().next().unwrap()) =>
                    {
                        None
                    }
                    c if c.is_uppercase() => Some(((*neigh, remaining_keys.clone()), 1)),
                    c if c.is_lowercase() && remaining_keys.contains(c) => Some((
                        (
                            *neigh,
                            remaining_keys.iter().cloned().filter(|k| k != c).collect(),
                        ),
                        1,
                    )),
                    c if c.is_lowercase() => Some(((*neigh, remaining_keys.clone()), 1)),
                    c => {
                        panic!("unknown {c}, neigh: {neigh:?}, remaining_keys: {remaining_keys:?}")
                    }
                }
            })
            .collect()
    }

    fn heuristic(&self, (_position, remaining_keys): &(Point, Vec<char>)) -> i64 {
        if remaining_keys.is_empty() {
            0
        } else {
            1
        }
    }

    fn success_end(&self, (_position, remaining_keys): &(Point, Vec<char>)) -> bool {
        remaining_keys.is_empty()
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().flatten().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = Point::new(x as i64, y as i64);
                solution.set_cell(point, c);
            }
        }
        Ok(solution)
    }
}
