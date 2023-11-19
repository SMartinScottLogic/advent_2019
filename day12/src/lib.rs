use std::{
    io::{BufRead, BufReader},
    ops::{AddAssign, Index, IndexMut},
};

use itertools::Itertools;
use tracing::{debug, info};
use utils::math::lowest_common_multiple_3;

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
        let lcm = lowest_common_multiple_3(period_x * 2, period_y * 2, period_z * 2);
        info!(period_x, period_y, period_z, lcm, "p2");
        // Implement for problem
        Ok(lcm as f64)
    }
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
                for i in 0..3 {
                    if moon_a.position[i] < moon_b.position[i] {
                        moon_b.velocity[i] -= 1.0;
                    }
                    if moon_a.position[i] > moon_b.position[i] {
                        moon_b.velocity[i] += 1.0;
                    }
                }
            }
        }
    }

    fn apply_velocity(moons: &mut Vec<State<f64>>) {
        for moon_id in 0..moons.len() {
            let moon = moons.get_mut(moon_id).unwrap();
            moon.position += &moon.velocity;
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
impl<T> AddAssign<&Vector3<T>> for Vector3<T>
where
    T: std::ops::AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: &Vector3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl<T> Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }
}
impl<T> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable!(),
        }
    }
}
