use std::{io::{BufRead, BufReader}, collections::{HashMap, HashSet}};

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
        // Implement for problem
        Ok(0)
    }
}

impl Solution {
    fn add_direct_orbit(&mut self, child: &str, parent: &str) {
        let entry = self.direct_orbits.entry(parent.to_string()).or_default();
        entry.insert(child.to_string());
    }

    fn find_root(&self) -> String {
        let children = self.direct_orbits.values().flat_map(|v| v.iter()).collect::<HashSet<_>>();
        let parents = self.direct_orbits.keys().collect::<HashSet<_>>();

        let root = parents.difference(&children).map(|v| v.to_owned()).collect::<HashSet<_>>();
        debug!(children = debug(&children), parents = debug(&children), root = debug(&root), "t");
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
                if start == "E" {
                    debug!(start, count, "probe");
                }
                (count, num_children)
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
