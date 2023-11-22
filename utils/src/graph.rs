use std::cmp::Eq;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::Copy;
use tracing::debug;

fn breadth_first_search_build_path<'a, N>(
    mut position: &'a N,
    path_fragments: &'a HashMap<N, N>,
) -> VecDeque<N>
where
    N: Debug + PartialEq + Eq + Hash + Clone,
{
    debug!(
        path_fragments = debug(&path_fragments),
        position = debug(position),
        "build path"
    );
    let mut total_path = VecDeque::new();
    while let Some(current) = path_fragments.get(position) {
        total_path.push_front(position.clone());
        position = current;
    }
    total_path
}

pub fn breadth_first_search<'a, N, IE, GN>(
    start: N,
    get_neighbours: GN,
    is_end: IE,
) -> Option<(N, VecDeque<N>)>
where
    N: Debug + PartialEq + Eq + Hash + Clone + 'a,
    IE: Fn(&N) -> bool,
    GN: Fn(&N) -> Vec<N>,
{
    let mut have_seen = HashSet::new();
    have_seen.insert(start.clone());
    let mut queue = VecDeque::new();
    queue.push_back(start.clone());
    let mut path_fragments = HashMap::new();

    while let Some(current) = queue.pop_front() {
        debug!(
            current = debug(&current),
            queue = debug(&queue),
            have_seen = debug(&have_seen),
            "popped"
        );
        if is_end(&current) {
            return Some((
                current.clone(),
                breadth_first_search_build_path(&current, &path_fragments),
            ));
        }
        for neighbour in get_neighbours(&current) {
            if have_seen.contains(&neighbour) {
                debug!(
                    current = debug(&current),
                    neighbour = debug(&neighbour),
                    "have seen"
                );
                continue;
            }
            have_seen.insert(neighbour.clone());
            queue.push_back(neighbour.clone());
            path_fragments.insert(neighbour.clone(), current.clone());
        }
    }
    None
}

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
    N: Debug + Eq + Copy + Hash,
    R: Debug + std::cmp::PartialOrd + Copy + std::ops::Add<Output = R> + HasOne,
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
