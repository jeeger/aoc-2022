use std::cmp::{max, min};

use crate::Point;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        if p1.x != p2.x && p1.y != p2.y {
            panic!("Can only create orthogonal lines.");
        }
        Line { p1: *p1, p2: *p2 }
    }

    pub fn on(&self, p: &Point) -> bool {
        (p.x == self.p1.x && p.y >= min(self.p1.y, self.p2.y) && p.y <= max(self.p1.y, self.p2.y)) ||
            (p.y == self.p1.y && p.x >= min(self.p1.x, self.p2.x) && p.x <= max(self.p1.x, self.p2.x))
    }

    pub fn on_infinite(&self, p: &Point) -> bool {
        (p.x == self.p1.x && p.x == self.p2.x) ||
            (p.y == self.p1.y && p.y == self.p2.y)
    }
}

#[cfg(test)]
mod test {
    use crate::{Line, Point};


    #[test]
    fn test_line_on() {
        let l = Line::new(&Point::new(0, 0), &Point::new(5, 0));
        assert!(l.on(&Point::new(1, 0)));
        assert!(!l.on(&Point::new(0, 1)));
    }
}
