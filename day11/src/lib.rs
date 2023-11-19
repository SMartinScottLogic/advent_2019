use std::io::{BufRead, BufReader};

use tracing::debug;
use utils::Matrix;

pub type ResultType = i64;

mod intcode;

#[derive(Debug, Default)]
pub struct Solution {
    entries: Vec<i64>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let hull = self.run(0);
        // Implement for problem
        Ok(hull.len().try_into().unwrap())
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let hull = self.run(1);
        hull.display_with_mapping(|v| {
            match v {
                0 => ".",
                1 => "#",
                _ => panic!(),
            }
            .to_owned()
        });
        // Implement for problem
        Ok(hull.len().try_into().unwrap())
    }
}

impl Solution {
    fn add_entry(&mut self, value: i64) {
        self.entries.push(value);
    }

    fn run(&self, first_panel_color: i64) -> Matrix<i64> {
        let mut cpu = intcode::Cpu::new(0, &self.entries);
        let mut x = 0;
        let mut y = 0;
        let mut facing = Direction::North;
        let mut hull = Matrix::new();
        hull.set(x, y, first_panel_color);
        cpu.input(1);
        while !cpu.has_halted() {
            cpu.execute();
            if cpu.needs_input() {
                let color = hull.get(x, y).unwrap_or(&0);
                cpu.input(*color);
            }
            match cpu.output().len() {
                2 => {
                    let v = cpu.take_output();
                    let [color, turn] = v[..] else { panic!() };
                    debug!(color, turn, values = debug(v), "painting");
                    // color
                    hull.set(x, y, color);
                    // turn
                    facing = match facing {
                        Direction::North if turn == 0 => Direction::West,
                        Direction::West if turn == 0 => Direction::South,
                        Direction::South if turn == 0 => Direction::East,
                        Direction::East if turn == 0 => Direction::North,
                        Direction::North if turn == 1 => Direction::East,
                        Direction::East if turn == 1 => Direction::South,
                        Direction::South if turn == 1 => Direction::West,
                        Direction::West if turn == 1 => Direction::North,
                        _ => panic!("invalid turn"),
                    };
                    // step
                    match facing {
                        Direction::North => y -= 1,
                        Direction::West => x -= 1,
                        Direction::South => y += 1,
                        Direction::East => x += 1,
                    };
                }
                1 | 0 => {}
                _ => {
                    panic!("overrun output")
                }
            }
            if cpu.output().len() > 2 {
                panic!();
            }
        }
        debug!(cpu = debug(&cpu), hull_size = hull.len(), "inspect");
        hull
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

enum Direction {
    North,
    West,
    South,
    East,
}
