use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    io::{BufRead, BufReader},
};

use tracing::debug;
use utils::Point;

mod intcode;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    entries: Vec<i64>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut cpu = intcode::Cpu::new(0, &self.entries);
        let mut current_direction = Direction::North;
        let mut position = Point::new(0, 0);
        let mut known = HashMap::new();
        known.insert(position, Block::Gap);
        'driver: loop {
            cpu.execute();
            if !cpu.output().is_empty() {
                for output in cpu.take_output() {
                    let probe_position = match current_direction {
                        Direction::North => position + Point::N,
                        Direction::South => position + Point::S,
                        Direction::East => position + Point::E,
                        Direction::West => position + Point::W,
                    };
                    let block = Block::from_intcode(output);
                    known.insert(probe_position, block.clone());
                    match block {
                        Block::Gap | Block::Oxygen => position = probe_position,
                        Block::Wall => {}
                        Block::None => unreachable!(),
                    };
                    debug!(
                        output,
                        direction = debug(&current_direction),
                        position = debug(position),
                        "progress"
                    );
                }
            }
            if cpu.needs_input() {
                if let Some(path) =
                    self.calculate_path(position, &known, |_, block| *block == Block::None)
                {
                    debug!(
                        path = debug(&path),
                        position = debug(position),
                        "computed path"
                    );
                    current_direction = Self::calculate_direction(&position, path.front().unwrap());
                    cpu.input(current_direction.to_intcode());
                } else {
                    // finished, I hope
                    break 'driver;
                }
            }
            if cpu.has_halted() {
                panic!();
            }
        }
        // Implement for problem
        let path = self
            .calculate_path(Point::new(0, 0), &known, |_, block| *block == Block::Oxygen)
            .unwrap();

        let path = path
            .iter()
            .map(|p| (p, known.get(p).cloned().unwrap_or_default()))
            .collect::<Vec<_>>();
        debug!(path = debug(&path), "optimal path?");
        Ok(path.len().try_into().unwrap())
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut cpu = intcode::Cpu::new(0, &self.entries);
        let mut current_direction = Direction::North;
        let mut position = Point::new(0, 0);
        let mut known = HashMap::new();
        known.insert(position, Block::Gap);
        'driver: loop {
            cpu.execute();
            if !cpu.output().is_empty() {
                for output in cpu.take_output() {
                    let probe_position = match current_direction {
                        Direction::North => position + Point::N,
                        Direction::South => position + Point::S,
                        Direction::East => position + Point::E,
                        Direction::West => position + Point::W,
                    };
                    let block = Block::from_intcode(output);
                    known.insert(probe_position, block.clone());
                    match block {
                        Block::Gap | Block::Oxygen => position = probe_position,
                        Block::Wall => {}
                        Block::None => unreachable!(),
                    };
                    debug!(
                        output,
                        direction = debug(&current_direction),
                        position = debug(position),
                        "progress"
                    );
                }
            }
            if cpu.needs_input() {
                if let Some(path) =
                    self.calculate_path(position, &known, |_, block| *block == Block::None)
                {
                    debug!(
                        path = debug(&path),
                        position = debug(position),
                        "computed path"
                    );
                    current_direction = Self::calculate_direction(&position, path.front().unwrap());
                    cpu.input(current_direction.to_intcode());
                } else {
                    // finished, I hope
                    break 'driver;
                }
            }
            if cpu.has_halted() {
                panic!();
            }
        }

        let (oxygen_pos, _) = known.iter().find(|(_, b)| *b == &Block::Oxygen).unwrap();

        let mut depth = 0;
        let mut current_depth = VecDeque::new();
        current_depth.push_back(oxygen_pos.to_owned());
        let mut seen = HashSet::new();
        seen.insert(oxygen_pos.to_owned());

        while !current_depth.is_empty() {
            let mut next_depth = VecDeque::new();
            while let Some(p) = current_depth.pop_front() {
                for neighbour in p.cardinal() {
                    if known.get(&neighbour) == Some(&Block::Gap) && !seen.contains(&neighbour) {
                        seen.insert(neighbour);
                        next_depth.push_back(neighbour);
                    }
                }
            }
            depth += 1;
            current_depth = next_depth;
        }

        Ok(depth - 1)
    }
}

impl Solution {
    fn add_entry(&mut self, entry: i64) {
        self.entries.push(entry);
    }

    fn calculate_direction(from: &Point, to: &Point) -> Direction {
        if from.x() < to.x() {
            Direction::East
        } else if from.x() > to.x() {
            Direction::West
        } else if from.y() < to.y() {
            Direction::South
        } else if from.y() > to.y() {
            Direction::North
        } else {
            panic!("no direction to travel")
        }
    }

    fn calculate_path<F>(
        &self,
        position: Point,
        known: &HashMap<Point, Block>,
        filter: F,
    ) -> Option<VecDeque<Point>>
    where
        F: Fn(Point, &Block) -> bool,
    {
        let mut positions = VecDeque::new();
        positions.push_back(position);

        let mut visited = Vec::new();
        visited.push(position);

        let mut path_fragments = HashMap::new();

        while let Some(current) = positions.pop_front() {
            let tile = known.get(&current).cloned().unwrap_or_default();
            // target - calculate path
            if filter(current, &tile) {
                debug!(
                    current = debug(current),
                    position = debug(position),
                    known = debug(known),
                    tile = debug(tile),
                    "found"
                );
                let path = self.build_path(path_fragments, current);
                return Some(path);
            }
            if tile != Block::Wall {
                for dir in current.cardinal() {
                    if visited.contains(&dir) {
                        continue;
                    }
                    visited.push(dir);
                    positions.push_back(dir);
                    path_fragments.insert(dir, current);
                }
            }
        }
        None
    }

    fn build_path(
        &self,
        path_fragments: HashMap<Point, Point>,
        mut position: Point,
    ) -> VecDeque<Point> {
        debug!(
            path_fragments = debug(&path_fragments),
            position = debug(position),
            "build path"
        );
        let mut total_path = VecDeque::new();
        while let Some(&current) = path_fragments.get(&position) {
            total_path.push_front(position);
            position = current;
        }
        total_path
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

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn to_intcode(&self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::East => 4,
            Direction::West => 3,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
enum Block {
    Wall,
    Gap,
    Oxygen,
    #[default]
    None,
}
impl Block {
    fn from_intcode(code: i64) -> Self {
        match code {
            0 => Self::Wall,
            1 => Self::Gap,
            2 => Self::Oxygen,
            _ => unreachable!(),
        }
    }
}
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Block::Wall => "Wall",
            Block::Gap => "Gap",
            Block::Oxygen => "Oxygen",
            Block::None => "None",
        };
        write!(f, "${s}")
    }
}
