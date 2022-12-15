use std::cmp::{max, min};
use std::fmt::Debug;
use std::collections::HashMap;

use crate::Point;

pub struct SparseMap<T> {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    map: HashMap<Point, T>,
}

impl <T> SparseMap<T> where
    T: Debug + Default + Copy
{
    pub fn new() -> Self {
        Self {
            min_x: isize::MAX,
            max_x: isize::MIN,
            min_y: isize::MAX,
            max_y: isize::MIN,
            map: HashMap::new(),
        }
    }

    pub fn at(&self, p: &Point) -> T {
        *self.map.get(p).unwrap_or(&Default::default())
    }

    pub fn bounds(&self) -> (Point, Point) {
        (
            Point::new(self.min_x, self.min_y),
            Point::new(self.max_x, self.max_y),
        )
    }

    pub fn put(&mut self, p: &Point, v: T) {
        self.min_x = min(p.x, self.min_x);
        self.max_x = max(p.x, self.max_x);
        self.min_y = min(p.y, self.min_y);
        self.max_y = max(p.y, self.max_y);
        self.map.insert(*p, v);
    }

    pub fn is_empty(&self, p: &Point) -> bool {
        !self.map.contains_key(p)
    }
}

