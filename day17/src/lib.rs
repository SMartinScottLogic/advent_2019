use std::io::{BufRead, BufReader};

use tracing::debug;
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
                35 => {
                    image.set(x, y, 1);
                    x += 1;
                }
                46 => {
                    image.set(x, y, 0);
                    x += 1;
                }
                10 => {
                    x = 0;
                    y += 1
                }
                _ => {
                    image.set(x, y, 1);
                    x += 1;
                }
            };
        }

        let mut total: i64 = 0;
        for ((x, y), _) in image.sparse_iter() {
            // count neighbours
            let p = Point::new(*x as i64, *y as i64);
            let c = p
                .cardinal()
                .iter()
                .map(|n| {
                    image
                        .get(n.x().try_into().unwrap(), n.y().try_into().unwrap())
                        .unwrap_or(&0)
                })
                .sum::<usize>();
            if c > 3 {
                debug!(intersection = debug(p), c, "intersection");
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
                '#' => {
                    image.set(x, y, 1);
                    x += 1;
                }
                '.' => {
                    image.set(x, y, 0);
                    x += 1;
                }
                '\n' => {
                    x = 0;
                    y += 1
                }
                '^' => {
                    position = Some(Point::new(x as i64, y as i64));
                    direction = Some(Point::N);
                    image.set(x, y, 2);
                    x += 1;
                }
                '>' => {
                    position = Some(Point::new(x as i64, y as i64));
                    direction = Some(Point::E);
                    image.set(x, y, 2);
                    x += 1;
                }
                'v' => {
                    position = Some(Point::new(x as i64, y as i64));
                    direction = Some(Point::S);
                    image.set(x, y, 2);
                    x += 1;
                }
                '<' => {
                    position = Some(Point::new(x as i64, y as i64));
                    direction = Some(Point::W);
                    image.set(x, y, 2);
                    x += 1;
                }
                c => panic!("unexpected char '{c}'"),
            };
        }

        /*
        image.display_with_mapping(|v| match v {
            0 => ".",
            1 => "#",
            2 => "X",
            _ => panic!(),
        }.to_string());
        */

        debug!(
            position = debug(position),
            direction = debug(direction),
            "start"
        );

        let mut position = position.unwrap();
        let mut direction = direction.unwrap();

        let mut commands = String::new();
        'outer: loop {
            let mut rotations = 0;
            loop {
                if rotations == 4 {
                    break 'outer;
                }
                if rotations != 2 {
                    if let Some(1) = image.get(
                        (position.x() + direction.x()) as isize,
                        (position.y() + direction.y()) as isize,
                    ) {
                        break;
                    }
                }
                rotations += 1;
                direction = Point::new(-direction.y(), direction.x());
            }
            let command = match rotations {
                1 => 'R',
                3 => 'L',
                _ => unreachable!(),
            };
            commands.push(',');
            commands.push(command);

            let mut steps = 0;
            while let Some(1) = image.get(
                (position.x() + direction.x()) as isize,
                (position.y() + direction.y()) as isize,
            ) {
                position += direction;
                steps += 1;
            }
            commands.push_str(&format!(",{steps}"));
            debug!(
                position = debug(position),
                direction = debug(direction),
                rotations,
                steps,
                "facing"
            );
        }

        debug!(commands, "master");
        // Created manually by inspection
        let a = "R,8,L,10,L,12,R,4";
        let b = "R,8,L,12,R,4,R,4";
        let c = "R,8,L,10,R,8";
        let commands = commands
            .replace(a, "A")
            .replace(b, "B")
            .replace(c, "C")
            .chars()
            .skip(1)
            .collect::<String>();
        debug!(commands, "master");

        let mut entries = self.entries.clone();
        entries[0] = 2;
        let mut cpu = intcode::Cpu::new(0, &entries);

        for c in commands.chars() {
            cpu.input(c as i64);
        }
        cpu.input(10);
        for c in a.chars() {
            cpu.input(c as i64);
        }
        cpu.input(10);
        for c in b.chars() {
            cpu.input(c as i64);
        }
        cpu.input(10);
        for c in c.chars() {
            cpu.input(c as i64);
        }
        cpu.input(10);
        cpu.input(110);
        cpu.input(10);

        loop {
            cpu.execute();
            if cpu.needs_input() {
                panic!();
            }
            if cpu.has_halted() {
                break;
            }
        }
        let r = cpu.take_output().pop().unwrap();
        // Implement for problem
        Ok(r)
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
