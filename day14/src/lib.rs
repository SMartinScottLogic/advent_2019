use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use regex::Regex;
use tracing::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    reactions: Vec<Reaction>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let ore = self.required_ore(1);
        // Implement for problem
        Ok(ore)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let target_fuel: ResultType = 1_000_000_000_000;
        let cost_one = self.required_ore(1);

        // left is current max which CAN deliver
        // right is current min which CAN'T deliver
        let mut left = target_fuel / cost_one;
        let mut right = left * 2;

        while right - left > 1 {
            let mid = (right + left) / 2;
            let required = self.required_ore(mid);
            if required < target_fuel {
                left = mid;
            } else {
                right = mid;
            }
        }
        // Implement for problem
        Ok(left)
    }
}

impl Solution {
    fn add_reaction(&mut self, reaction: Reaction) {
        self.reactions.push(reaction);
    }

    fn use_in_hand(reagent: &Reagent, in_hand: &mut HashMap<String, ResultType>) -> ResultType {
        match in_hand.get_mut(&reagent.name) {
            None => reagent.count,
            Some(c) if *c > reagent.count => {
                *c -= reagent.count;
                0
            }
            Some(c) => {
                let still_need = reagent.count - *c;
                *c = 0;
                still_need
            }
        }
    }

    fn required_ore(&self, desired_fuel: ResultType) -> ResultType {
        let mut ore: ResultType = 0;
        let mut in_hand = HashMap::new();
        let mut required = vec![Reagent {
            name: String::from("FUEL"),
            count: desired_fuel,
        }];
        while let Some(requirement) = required.pop() {
            if requirement.name == "ORE" {
                ore += requirement.count;
                continue;
            }
            let required_count = Self::use_in_hand(&requirement, &mut in_hand);
            if required_count == 0 {
                continue;
            }
            // Find reaction which produces the requirement
            let reaction = self
                .reactions
                .iter()
                .find(|r| r.output.name == requirement.name)
                .unwrap();
            // How many reactions MUST we have to produce AT LEAST the requirements
            let multiplier =
                (required_count as f64 / reaction.output.count as f64).ceil() as ResultType;
            let remainder = reaction.output.count * multiplier - required_count;
            *in_hand.entry(reaction.output.name.clone()).or_insert(0) += remainder;
            debug!(
                requirement = debug(requirement),
                required_count,
                reaction = debug(reaction),
                multiplier,
                remainder,
                in_hand = debug(&in_hand),
                "find reaction"
            );
            for i in &reaction.inputs {
                required.push(Reagent {
                    name: i.name.clone(),
                    count: i.count * multiplier,
                });
            }
        }
        ore
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> std::result::Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().flatten() {
            let reaction = line.into();
            solution.add_reaction(reaction);
            // Implement for problem
        }
        Ok(solution)
    }
}

#[derive(Debug)]
struct Reaction {
    inputs: Vec<Reagent>,
    output: Reagent,
}
#[derive(Debug)]
struct Reagent {
    name: String,
    count: ResultType,
}
impl From<String> for Reaction {
    fn from(value: String) -> Self {
        let r = Regex::new(r"^(?P<count>\d+) (?P<name>.*)$").unwrap();
        let (lhs, rhs) = value.split_once(" => ").unwrap();
        let inputs = lhs
            .split(", ")
            .map(|v| {
                r.captures(v)
                    .map(|c| {
                        let name = c.name("name").unwrap().as_str().to_string();
                        let count = c.name("count").unwrap().as_str().parse().unwrap();
                        Reagent { name, count }
                    })
                    .unwrap()
            })
            .collect();
        let output = r
            .captures(rhs)
            .map(|c| {
                let name = c.name("name").unwrap().as_str().to_string();
                let count = c.name("count").unwrap().as_str().parse().unwrap();
                Reagent { name, count }
            })
            .unwrap();
        Self { inputs, output }
    }
}
