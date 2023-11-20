use std::io::{BufRead, BufReader};

use tracing::info;
use utils::{Matrix, Point};

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
        let mut cpu = intcode::Cpu::new(0, &self.entries);
        loop {
            cpu.execute();
            if cpu.needs_input() {
                panic!();
            }
            if cpu.has_halted() {
                break;
            }
        }
        let mut image = Matrix::new();
        let output = cpu.take_output();
        let mut x = 0;
        let mut y = 0;
        for c in output {
            match c {
                35 => {image.set(x, y, 1); x += 1;},
                46 => {image.set(x, y, 0); x += 1;},
                10 => {x = 0; y+= 1},
                _ => {image.set(x, y, 1); x += 1;},
            };
        }

        let mut total: i64 = 0;
        for ((x, y), v) in image.sparse_iter() {
            // count neighbours
            let p = Point::new(*x as i64, *y as i64);
            let c = p.cardinal().iter()
            .map(|n| image.get(n.x().try_into().unwrap(), n.y().try_into().unwrap()).unwrap_or(&0))
            .sum::<usize>();
            if c > 3 {
                info!(intersection = debug(p), c, "intersection");
                total += *x as i64 * *y as i64;
            }
        }
    // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut cpu = intcode::Cpu::new(0, &self.entries);
        loop {
            cpu.execute();
            if cpu.needs_input() {
                panic!();
            }
            if cpu.has_halted() {
                break;
            }
        }
        let mut image = Matrix::new();
        let output = cpu.take_output();
        let mut x = 0;
        let mut y = 0;
        let mut position = None;
        let mut direction = None;
        for c in output {
            match c as u8 as char {
                '#' => {image.set(x, y, 1); x += 1;},
                '.' => {image.set(x, y, 0); x += 1;},
                '\n' => {x = 0; y+= 1},
                '^' => { position = Some(Point::new(x as i64, y as i64)); direction = Some(Point::N); image.set(x, y, 1); x += 1; }
                '>' => { position = Some(Point::new(x as i64, y as i64)); direction = Some(Point::E); image.set(x, y, 1); x += 1; }
                'v' => { position = Some(Point::new(x as i64, y as i64)); direction = Some(Point::S); image.set(x, y, 1); x += 1; }
                '<' => { position = Some(Point::new(x as i64, y as i64)); direction = Some(Point::W); image.set(x, y, 1); x += 1; }
                c => panic!("unexpected char '{c}'"),
            };
        }

        info!(position = debug(position), direction = debug(direction), "start");

        let position = position.unwrap();
        let mut direction = direction.unwrap();

        let mut rotations = 0;
        loop {
            if let Some(1) = image.get((position.x() + direction.x()) as isize, (position.y() + direction.y()) as isize) {
                break;
            }
            rotations = 1;
            direction = Point::new(direction.y(), -direction.x());
        };

        info!(position = debug(position), direction = debug(direction), rotations, "facing");

        let mut entries = self.entries.clone();
        entries[0] = 2;
        let mut cpu = intcode::Cpu::new(0, &entries);
        loop {
        cpu.execute();
        if cpu.needs_input() {
            panic!();
        }
        if cpu.has_halted() {
            break;
        }
        }
        // Implement for problem
        Ok(0)
    }
}

impl Solution {
    fn add_entry(&mut self, entry: i64) {
        self.entries.push(entry);
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
