use std::{
    cmp::max,
    f64::consts::PI,
    fmt::{Debug, Display},
    io::{BufRead, BufReader},
};
use tracing::{debug, info};
use utils::{math::greatest_common_divisor, Matrix};

pub type ResultType = i64;

#[derive(Default)]
pub struct Solution {
    asteroids: Matrix<i64>,
}
impl Debug for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Solution")
            //.field("asteroids", &self.asteroids.len())
            .finish()
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let (maxx, maxy) = self.asteroids.dimensions();
        let mut max_count = 0;
        let mut best_x = 0;
        let mut best_y = 0;
        for y in 0..=maxy {
            for x in 0..=maxx {
                if let Some(1) = self.asteroids.get(x, y) {
                    let count = self.count_visible(x, y);
                    if count > max_count {
                        max_count = count;
                        best_x = x;
                        best_y = y;
                    }
                    //break 'end;
                }
            }
        }
        info!(best_x, best_y, max_count, "answer");
        // Implement for problem
        Ok(max_count)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let (maxx, maxy) = self.asteroids.dimensions();
        let mut max_count = 0;
        let mut best_x = 0;
        let mut best_y = 0;
        for y in 0..=maxy {
            for x in 0..=maxx {
                if let Some(1) = self.asteroids.get(x, y) {
                    let count = self.count_visible(x, y);
                    if count > max_count {
                        max_count = count;
                        best_x = x;
                        best_y = y;
                    }
                    //break 'end;
                }
            }
        }
        info!("new base: ({},{})", best_x, best_y);

        let mut destroyed = Vec::new();
        let mut asteroids = self.asteroids.clone();
        loop {
            destroyed.push((-1, -1, -1.0));
            let visibility = Self::compute_visible(&asteroids, best_x, best_y);
            let mut v = visibility
                .sparse_iter()
                .filter_map(|((x, y), status)| {
                    debug!(x, y, status = debug(status), "probe1");
                    match status {
                        Status::Visible => Some((x, y)),
                        _ => None,
                    }
                })
                .map(|(x, y)| {
                    let dx = *x as f64 - best_x as f64;
                    let dy = *y as f64 - best_y as f64;
                    // TODO compute angle arccos( (a.b)/(|a||b|))
                    let angle_y_axis = angle_y(dx, dy);
                    debug!(x, y, dx, dy, angle_y_axis, "probe2");
                    (angle_y_axis, x, y, dx, dy)
                })
                .collect::<Vec<_>>();
            v.sort_by(|(angle_a, ..), (angle_b, ..)| angle_a.partial_cmp(angle_b).unwrap());
            if v.is_empty() {
                break;
            }
            debug!(len = v.len(), v = debug(&v), "angles");
            for (a, x, y, ..) in v {
                debug!(x, y, "probe3");
                destroyed.push((*x, *y, a));
                asteroids.set(*x, *y, 0);
            }
        }
        destroyed.iter().enumerate().for_each(|(id, (x, y, a))| {
            info!(x, y, dx = x - best_x, dy = y - best_y, a, id, "all");
        });
        let (a_x, a_y, ..) = destroyed.get(199).unwrap_or(&(0, 0, 0.0));
        info!(destroyed = debug(&destroyed), a_x, a_y, "destroyed order");
        Ok((100 * a_x + a_y).try_into().unwrap())
    }
}

#[derive(Default, Debug, Clone)]
enum Status {
    #[default]
    Unknown,
    Visible,
    Covered,
    Root,
}
impl Display for Status {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Solution {
    fn add_asteroid(&mut self, x: usize, y: usize) {
        self.asteroids
            .set(x.try_into().unwrap(), y.try_into().unwrap(), 1);
    }

    fn compute_visible(asteroids: &Matrix<i64>, x: isize, y: isize) -> Matrix<Status> {
        let (max_x, max_y) = asteroids.dimensions();
        let max_delta = max(max(x, max_x - x), max(y, max_y - y));
        debug!(x, y, max_x, max_y, max_delta, "delta");
        let mut status = Matrix::new();
        for delta in 1..=max_delta {
            for dy in -delta..=delta {
                for dx in -delta..=delta {
                    if dy.abs() != delta && dx.abs() != delta {
                        continue;
                    }
                    let px = x + dx;
                    let py = y + dy;
                    if px < 0 || px > max_x {
                        continue;
                    }
                    if py < 0 || py > max_y {
                        continue;
                    }
                    if let Some(1) = asteroids.get(px, py) {
                        if status.get(px, py).is_none() {
                            debug!(x, y, px, py, delta, max_delta, max_x, max_y, "sight");
                            status.set(px, py, Status::Visible);
                            Self::mark_unseeable(
                                asteroids,
                                x,
                                y,
                                max_x,
                                max_y,
                                dx,
                                dy,
                                &mut status,
                                Status::Covered,
                            );
                        }
                    }
                }
            }
        }
        status.set(x, y, Status::Root);
        status
    }

    fn count_visible(&self, x: isize, y: isize) -> i64 {
        let (max_x, max_y) = self.asteroids.dimensions();
        let status = Self::compute_visible(&self.asteroids, x, y);
        //status.display();

        let mut count = 0;
        for y in 0..=max_y {
            for x in 0..=max_x {
                count += match status.get(x, y) {
                    None => 0,
                    Some(Status::Visible) => 1,
                    Some(Status::Root) => 0,
                    Some(Status::Covered) => 0,
                    Some(v) => panic!("unexpected status {v}"),
                }
            }
        }
        count
    }

    fn mark_unseeable(
        asteroids: &Matrix<i64>,
        x: isize,
        y: isize,
        max_x: isize,
        max_y: isize,
        dx: isize,
        dy: isize,
        status: &mut Matrix<Status>,
        value: Status,
    ) {
        let gcd: isize =
            greatest_common_divisor(dx.abs().try_into().unwrap(), dy.abs().try_into().unwrap())
                .try_into()
                .unwrap();
        let step_x = dx / gcd;
        let step_y = dy / gcd;
        let mut px = x + dx;
        let mut py = y + dy;
        loop {
            px += step_x;
            py += step_y;
            if px < 0 || px > max_x {
                break;
            }
            if py < 0 || py > max_y {
                break;
            }
            if let Some(1) = asteroids.get(px, py) {
                if status.get(px, py).is_none() {
                    status.set(px, py, value.clone());
                }
            }
            debug!(x, y, max_x, max_y, px, py, gcd, "probe");
        }
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();

        for (y, line) in reader.lines().flatten().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    solution.add_asteroid(x, y);
                }
            }
            // Implement for problem
        }
        Ok(solution)
    }
}

fn angle_y(x: f64, y: f64) -> f64 {
    let mut angle_y_axis = (y / (x * x + y * y).sqrt()).acos();
    if x < 0.0 {
        // q4
        angle_y_axis = 2.0 * PI - angle_y_axis;
    } else {
        angle_y_axis = PI - angle_y_axis;
    }
    angle_y_axis
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn assert_close(a: f64, b: f64) {
        println!("{a} ~ {b}");
        assert!((a - b).abs() < 1e-10);
    }

    #[test]
    fn test_angle_1() {
        let r = angle_y(0.0, 1.0);
        assert_close(r, PI);
    }
    #[test]
    fn test_angle_2() {
        let r = angle_y(1.0, 1.0);
        assert_close(r, PI * 3.0 / 4.0);
    }
    #[test]
    fn test_angle_3() {
        let r = angle_y(1.0, 0.0);
        assert_close(r, PI / 2.0);
    }
    #[test]
    fn test_angle_4() {
        let r = angle_y(1.0, -1.0);
        assert_close(r, PI / 4.0);
    }
    #[test]
    fn test_angle_5() {
        let r = angle_y(0.0, -1.0);
        assert_close(r, 0.0);
    }
    #[test]
    fn test_angle_6() {
        let r = angle_y(-1.0, -1.0);
        assert_close(r, PI + PI / 4.0);
    }
    #[test]
    fn test_angle_7() {
        let r = angle_y(-1.0, 0.0);
        assert_close(r, PI + PI / 2.0);
    }
    #[test]
    fn test_angle_8() {
        let r = angle_y(-1.0, 1.0);
        assert_close(r, PI + PI * 3.0 / 4.0);
    }
}
