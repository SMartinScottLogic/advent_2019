use std::io::{BufRead, BufReader};

use tracing::debug;

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    masses: Vec<i64>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut total_fuel = 0;
        for mass in &self.masses {
            let fuel = Self::calculate_fuel(mass);
            debug!(mass, fuel, "calculate fuel");
            total_fuel += fuel;
        }
        // Implement for problem
        Ok(total_fuel)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut total_fuel = 0;
        for mass in &self.masses {
            let mut mass = *mass;
            loop {
                let fuel = Self::calculate_fuel(&mass);
                debug!(mass, fuel, "calculate fuel");
                if fuel <= 0 {
                    break;
                }
                total_fuel += fuel;
                mass = fuel;
            }
            debug!(mass, total_fuel, "calculate fuel");
        }
        // Implement for problem
        Ok(total_fuel)
    }
}

impl Solution {
    fn add_mass(&mut self, mass: i64) {
        self.masses.push(mass);
    }

    fn calculate_fuel(mass: &i64) -> i64 {
        (mass / 3) - 2
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().flatten() {
            if let Ok(mass) = line.parse::<i64>() {
                solution.add_mass(mass);
            }
        }
        Ok(solution)
    }
}
