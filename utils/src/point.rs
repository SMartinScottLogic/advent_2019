use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub const N: Self = Self { x: 0, y: -1 };
    pub const S: Self = Self { x: 0, y: 1 };
    pub const E: Self = Self { x: 1, y: 0 };
    pub const W: Self = Self { x: -1, y: 0 };

    pub const NE: Self = Self { x: 1, y: -1 };
    pub const NW: Self = Self { x: -1, y: -1 };
    pub const SE: Self = Self { x: 1, y: 1 };
    pub const SW: Self = Self { x: -1, y: 1 };

    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn neighbours(&self) -> Vec<Self> {
        [
            Self::N,
            Self::E,
            Self::S,
            Self::W,
            Self::NE,
            Self::SE,
            Self::SW,
            Self::NW,
        ]
        .iter()
        .map(|d| *self + d)
        .collect()
    }

    pub fn cardinal(&self) -> Vec<Self> {
        [Self::N, Self::E, Self::S, Self::W]
            .iter()
            .map(|d| *self + d)
            .collect()
    }
}

impl Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Self> for Point {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Self> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<&Self> for Point {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::point::Point;

    #[test]
    fn test_add() {
        let a = Point::new(1, 2);
        let b = Point::new(-1, -1);
        assert_eq!(Point::new(0, 1), a + b);
        assert_eq!(Point::new(0, 1), b + a);
    }

    #[test]
    fn test_sub() {
        let a = Point::new(1, 2);
        let b = Point::new(-1, -1);
        assert_eq!(Point::new(2, 3), a - b);
        assert_eq!(Point::new(-2, -3), b - a);
    }

    #[test]
    fn test_cardinal() {
        let mut cardinal = Point::new(2, 2).cardinal();
        cardinal.sort();
        let mut expected = vec![
            Point::new(2, 1),
            Point::new(1, 2),
            Point::new(3, 2),
            Point::new(2, 3),
        ];
        expected.sort();

        assert_eq!(expected, cardinal);
    }

    #[test]
    fn test_neighbours() {
        let mut neighbours = Point::new(2, 2).neighbours();
        neighbours.sort();
        let mut expected = vec![
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(3, 1),
            Point::new(1, 2),
            Point::new(3, 2),
            Point::new(1, 3),
            Point::new(2, 3),
            Point::new(3, 3),
        ];
        expected.sort();

        assert_eq!(expected, neighbours);
    }
}
