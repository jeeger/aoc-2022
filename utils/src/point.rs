use std::cmp::max;
use std::cmp::min;
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

    pub fn orthogonal_range<'a>(
        self: &'a Point,
        p2: &'a Point,
    ) -> Box<dyn Iterator<Item = Point> + 'a> {
        if self.x != p2.x && self.y != p2.y {
            panic!("Can only generate orthogonal ranges!");
        };
        if self.x == p2.x {
            Box::new((min(self.y, p2.y)..max(self.y, p2.y) + 1).map(|y| Point::new(self.x, y)))
        } else {
            Box::new((min(self.x, p2.x)..max(self.x, p2.x) + 1).map(|x| Point::new(x, self.y)))
        }
    }

    pub fn points_at_dist<'a>(&'a self, d: isize) -> Box<dyn Iterator<Item = Point> + 'a> {
        Box::new((0..d + 1)
            .flat_map(move |offset| {
                vec![
                    Point::new(self.x + offset, self.y + (d - offset)),
                    Point::new(self.x - offset, self.y - (d - offset)),
                    Point::new(self.x + offset, self.y - (d - offset)),
                    Point::new(self.x - offset, self.y + (d - offset)),
                ]
            }))
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

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::Point;

    #[test]
    fn test_adjacent() {
        assert!(Point::new(1, 0).is_adjacent(&Point::new(2, 0)));
        assert!(Point::new(1, 0).is_adjacent(&Point::new(1, 1)));
        assert!(Point::new(1, 0).is_adjacent(&Point::new(0, 0)));
        assert!(Point::new(1, 1).is_adjacent(&Point::new(1, 0)));
        assert!(!Point::new(1, 1).is_adjacent(&Point::new(3, 1)));
    }

    #[test]
    fn test_point_range() {
        let p1 = Point::new(10, 20);
        let p2 = Point::new(15, 20);
        let p3 = Point::new(10, 23);
        assert_eq!(
            p1.orthogonal_range(&p2).collect::<Vec<Point>>(),
            vec![
                Point::new(10, 20),
                Point::new(11, 20),
                Point::new(12, 20),
                Point::new(13, 20),
                Point::new(14, 20),
                Point::new(15, 20)
            ]
        );
        assert_eq!(
            p2.orthogonal_range(&p1).collect::<Vec<Point>>(),
            vec![
                Point::new(10, 20),
                Point::new(11, 20),
                Point::new(12, 20),
                Point::new(13, 20),
                Point::new(14, 20),
                Point::new(15, 20)
            ]
        );
        assert_eq!(
            p1.orthogonal_range(&p3).collect::<Vec<Point>>(),
            vec![
                Point::new(10, 20),
                Point::new(10, 21),
                Point::new(10, 22),
                Point::new(10, 23),
            ]
        );
        assert_eq!(
            p3.orthogonal_range(&p1).collect::<Vec<Point>>(),
            vec![
                Point::new(10, 20),
                Point::new(10, 21),
                Point::new(10, 22),
                Point::new(10, 23),
            ]
        );
    }

    #[test]
    fn test_points_at_dist() {
        assert_eq!(
            Point::new(0, 0).points_at_dist(1).collect::<HashSet<Point>>(),
            HashSet::from([
                Point::new(-1, 0),
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(0, -1)
            ])
        );
        assert_eq!(
            Point::new(0, 0).points_at_dist(2).collect::<HashSet<Point>>(),
            HashSet::from([
                Point::new(-2, 0),
                Point::new(-1, -1),
                Point::new(0, -2),
                Point::new(1, -1),
                Point::new(2, 0),
                Point::new(1, 1),
                Point::new(0, 2),
                Point::new(-1, 1)
            ])
        );
    }
}
