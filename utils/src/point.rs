use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    pub fn is_adjacent(&self, other: &Self) -> bool {
        let (xdist, ydist) = self.vec_dist(other);
        xdist <= 1 && xdist >= -1 && ydist <= 1 && ydist >= -1
    }

    pub fn vec_dist(&self, other: &Self) -> (i32, i32) {
        (
            (self.x as i32) - (other.x as i32),
            (self.y as i32) - (other.y as i32),
        )
    }

    /*
    Taxicab distance between two points.
    */
    pub fn dist(&self, other: &Point) -> u32 {
        (((self.x as isize) - (other.x as isize)).abs()
            + ((self.y as isize) - (other.y as isize)).abs()) as u32
    }

    pub fn adjust(&mut self, xdiff: i32, ydiff: i32) {
        self.x = ((self.x as i32) + xdiff) as isize;
        self.y = ((self.y as i32) + ydiff) as isize;
    }
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Point::new(value.0, value.1)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}
