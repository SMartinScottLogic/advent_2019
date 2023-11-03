use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Matrix {
    data: HashMap<(isize, isize), i64>,
    max_x: isize,
    max_y: isize,
    min_x: isize,
    min_y: isize,
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&i64> {
        self.data.get(&(x, y))
    }

    pub fn set(&mut self, x: isize, y: isize, value: i64) {
        *self.data.entry((x, y)).or_insert(0) = value;
        self.max_x = max(self.max_x, x);
        self.max_y = max(self.max_y, y);
        self.min_x = min(self.min_x, x);
        self.min_y = min(self.min_y, y);
    }

    pub fn dimensions(&self) -> (isize, isize) {
        // TODO: Expand to include minima
        (self.max_x, self.max_y)
    }

    pub fn max_x(&self) -> isize {
        // TODO: Expand to include minima
        self.max_x
    }

    pub fn max_y(&self) -> isize {
        // TODO: Expand to include minima
        self.max_y
    }

    pub fn display(&self) {
        self.display_with_mapping(|v| format!("{v}"));
    }
    pub fn display_with_mapping<F>(&self, mapping: F)
    where
        F: Fn(i64) -> String,
    {
        for y in self.min_y..=self.max_y {
            let mut line = String::new();
            line.push_str(&format!("{} ", y));
            for x in self.min_x..=self.max_x {
                let v = self.get(x, y).unwrap_or(&0);
                let v = mapping(*v);
                line.push_str(&v);
            }
            println!("{line}");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn matrix() {
        let mut matrix = Matrix::new();
        matrix.set(1, 1, 1);
        let result = matrix.get(1, 1);
        assert_eq!(result, Some(&1i64));
    }
}
