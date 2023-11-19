use std::io::{BufRead, BufReader};

use itertools::Itertools;
use tracing::debug;

pub type ResultType = f64;

#[derive(Debug, Default)]
pub struct Solution {
    moons: Vec<State<f64>>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut moons = self.moons.clone();
        for step in 1..=1000 {
            Self::apply_gravity(&mut moons);
            Self::apply_velocity(&mut moons);
            debug!(step, moons = debug(&moons), "after");
        }
        Ok(Self::calculate_energy(&moons))
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut moons = self.moons.clone();
        let mut step = 1;
        let period_x = loop {
            Self::apply_gravity(&mut moons);
            Self::apply_velocity(&mut moons);
            debug!(step, moons = debug(&moons), "after");
            if moons.iter().map(|moon| moon.velocity.x.abs()).sum::<f64>() == 0.0 {
                break step;
            }
            step += 1;
        };
        let mut moons = self.moons.clone();
        let mut step = 1;
        let period_y = loop {
            Self::apply_gravity(&mut moons);
            Self::apply_velocity(&mut moons);
            debug!(step, moons = debug(&moons), "after");
            if moons.iter().map(|moon| moon.velocity.y.abs()).sum::<f64>() == 0.0 {
                break step;
            }
            step += 1;
        };
        let mut moons = self.moons.clone();
        let mut step = 1;
        let period_z = loop {
            Self::apply_gravity(&mut moons);
            Self::apply_velocity(&mut moons);
            debug!(step, moons = debug(&moons), "after");
            if moons.iter().map(|moon| moon.velocity.z.abs()).sum::<f64>() == 0.0 {
                break step;
            }
            step += 1;
        };
        let lcm = lcm3(period_x * 2, period_y * 2, period_z * 2);
        info!(period_x, period_y, period_z, lcm, "p2");
        // Implement for problem
        Ok(lcm as f64)
    }
}

// From https://doc.rust-lang.org/std/ops/trait.Div.html
// Euclid's two-thousand-year-old algorithm for finding the greatest common
// divisor.
fn gcd(x: u64, y: u64) -> u64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcm3(a: u64, b: u64, c: u64) -> u64 {
    lcm(a, lcm(b, c))
}

impl Solution {
    fn add_moon(&mut self, moon: State<f64>) {
        self.moons.push(moon);
    }

    fn apply_gravity(moons: &mut Vec<State<f64>>) {
        for moon_a_id in 0..moons.len() {
            for moon_b_id in 0..moons.len() {
                if moon_a_id == moon_b_id {
                    continue;
                }
                let moon_a = moons.get(moon_a_id).unwrap().clone();
                let moon_b = moons.get_mut(moon_b_id).unwrap();
                // X
                if moon_a.position.x < moon_b.position.x {
                    moon_b.velocity.x -= 1.0;
                }
                if moon_a.position.x > moon_b.position.x {
                    moon_b.velocity.x += 1.0;
                }
                // Y
                if moon_a.position.y < moon_b.position.y {
                    moon_b.velocity.y -= 1.0;
                }
                if moon_a.position.y > moon_b.position.y {
                    moon_b.velocity.y += 1.0;
                }
                // Z
                if moon_a.position.z < moon_b.position.z {
                    moon_b.velocity.z -= 1.0;
                }
                if moon_a.position.z > moon_b.position.z {
                    moon_b.velocity.z += 1.0;
                }
            }
        }
    }

    fn apply_velocity(moons: &mut Vec<State<f64>>) {
        for moon_id in 0..moons.len() {
            let moon = moons.get_mut(moon_id).unwrap();
            moon.position.x += moon.velocity.x;
            moon.position.y += moon.velocity.y;
            moon.position.z += moon.velocity.z;
        }
    }

    fn calculate_energy(moons: &[State<f64>]) -> f64 {
        moons
            .iter()
            .map(|moon| {
                let potential =
                    moon.position.x.abs() + moon.position.y.abs() + moon.position.z.abs();
                let kinetic = moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs();
                let total = potential * kinetic;
                debug!(potential, kinetic, total, "energy");
                total
            })
            .sum()
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().flatten() {
            if let Some((x, y, z)) = line
                .split(',')
                .map(|p| {
                    let (_, r) = p.split_once('=').unwrap();
                    r.replace('>', "").parse::<f64>().unwrap()
                })
                .collect_tuple()
            {
                solution.add_moon(State {
                    position: Vector3 { x, y, z },
                    velocity: Vector3::default(),
                })
            }
        }
        Ok(solution)
    }
}

#[derive(Default, Debug, Clone)]
struct State<T> {
    position: Vector3<T>,
    velocity: Vector3<T>,
}

#[derive(Default, Debug, Clone)]
struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}