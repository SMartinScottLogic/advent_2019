use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};

use tracing::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    direct_orbits: HashMap<String, HashSet<String>>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let root = self.find_root();
        let (answer, c) = self.count_all_paths(&root);
        debug!(answer, c, "result");
        // Implement for problem
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let root = self.find_root();
        let santa_parents = self.all_parents(&root, "SAN");
        let santa_parents = santa_parents
            .iter()
            .fold(HashMap::new(), |mut acc, (k, v)| {
                acc.insert(k, v);
                acc
            });
        let you_parents = self.all_parents(&root, "YOU");
        let you_parents = you_parents.iter().fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        });
        debug!(
            santa = debug(&santa_parents),
            you = debug(&you_parents),
            "both"
        );

        let mut joint = HashMap::new();
        for (p, d) in santa_parents {
            if let Some(d2) = you_parents.get(p) {
                joint.insert(p, d + *d2);
            }
        }
        debug!(joint = debug(&joint), "joint");

        let (_, d) = joint.iter().min_by_key(|(_, d)| *d).unwrap();

        // Implement for problem
        Ok(*d as ResultType)
    }
}

impl Solution {
    fn add_direct_orbit(&mut self, child: &str, parent: &str) {
        let entry = self.direct_orbits.entry(parent.to_string()).or_default();
        entry.insert(child.to_string());
    }

    fn find_root(&self) -> String {
        let children = self
            .direct_orbits
            .values()
            .flat_map(|v| v.iter())
            .collect::<HashSet<_>>();
        let parents = self.direct_orbits.keys().collect::<HashSet<_>>();

        let root = parents
            .difference(&children)
            .map(|v| v.to_owned())
            .collect::<HashSet<_>>();
        assert_eq!(1, root.len());
        root.iter().next().unwrap().to_owned().to_owned()
    }

    fn count_all_paths(&self, start: &str) -> (ResultType, ResultType) {
        match self.direct_orbits.get(start) {
            None => (0, 0),
            Some(children) => {
                let mut count = 0;
                let mut num_children = 0;
                for child in children {
                    let (inner_count, all_children) = self.count_all_paths(child);
                    count += inner_count;
                    count += all_children;
                    count += 1;
                    num_children += 1 + all_children;
                }
                (count, num_children)
            }
        }
    }

    fn all_parents(&self, start: &str, probe: &str) -> Vec<(String, usize)> {
        match self.direct_orbits.get(start) {
            None => vec![],
            Some(children) => {
                if children.contains(probe) {
                    vec![(start.to_string(), 0)]
                } else {
                    let mut distances = Vec::new();
                    for child in children {
                        let mut child_distances = self.all_parents(child, probe);
                        if !child_distances.is_empty() {
                            debug!(start, child_distances = debug(&child_distances), "p");
                            distances.push((
                                start.to_string(),
                                1 + child_distances.iter().map(|(_, d)| d).max().unwrap(),
                            ));
                            distances.append(&mut child_distances);
                        }
                    }
                    distances
                }
            }
        }
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().flatten() {
            // Implement for problem
            if let Some((l, r)) = line.split_once(')') {
                solution.add_direct_orbit(r, l);
            }
        }
        Ok(solution)
    }
}
