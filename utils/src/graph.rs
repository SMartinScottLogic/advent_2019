use std::collections::{HashMap, HashSet};

use tracing::debug;

pub fn dijkstra<N, IS, IE, GN, NEIGH, R>(
    nodes: &Vec<N>,
    initial_score: IS,
    get_neighbours: GN,
    is_end: IE,
) -> Option<R>
where
    IS: Fn(&N) -> Option<R>,
    GN: Fn(&N) -> NEIGH,
    IE: Fn(&N) -> bool,
    N: std::fmt::Debug + std::cmp::Eq + std::marker::Copy + std::hash::Hash,
    R: std::fmt::Debug
        + std::cmp::PartialOrd
        + std::marker::Copy
        + std::ops::Add<Output = R>
        + HasOne,
    NEIGH: std::iter::Iterator<Item = N>,
{
    let mut scores = HashMap::new();
    for node in nodes {
        if let Some(s) = initial_score(node) {
            scores.insert(*node, s);
        }
    }
    let mut visited = HashSet::new();
    let result = loop {
        // Find smallest, unvisited
        let mut bestnode = None;
        let mut bestscore = None;
        for (node, score) in scores.iter() {
            if !visited.contains(node) {
                match bestscore {
                    None => {
                        bestnode = Some(node.to_owned());
                        bestscore = Some(score.to_owned());
                    }
                    Some(s) if s > *score => {
                        bestnode = Some(*node);
                        bestscore = Some(score.to_owned());
                    }
                    Some(_) => {}
                }
            }
        }
        if bestnode.is_none() {
            break None;
        }
        let bestnode = bestnode.unwrap();
        let bestscore = bestscore.unwrap();
        visited.insert(bestnode);
        if is_end(&bestnode) {
            break Some(bestscore);
        }
        let neighbours = get_neighbours(&bestnode);
        for neighbour in neighbours {
            let n = neighbour;
            let score = scores.entry(n).or_insert(bestscore + R::one());
            if *score > bestscore + R::one() {
                *score = bestscore + R::one();
            }
        }
    };
    debug!("{:?}", scores);
    result
}

pub trait HasOne {
    fn one() -> Self;
}

impl HasOne for i64 {
    fn one() -> Self {
        1
    }
}
