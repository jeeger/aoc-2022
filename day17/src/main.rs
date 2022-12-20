extern crate nom;
use std::fs::read_to_string;

use utils::{Line, Point, SparseMap};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum JetDirection {
    Left,
    Right,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Shapes {
    HLine(Point),
    Cross(Point),
    Ell(Point),
    VLine(Point),
    Square(Point),
}

trait Shape {
    fn pos(&self) -> &Point;
    fn covers(&self, p: &Point) -> bool;
    fn touches(&self, p: &Line) -> bool;
    fn bounds(&self) -> (Point, Point);
    fn dropped(&self) -> Shapes;
    fn pushed(&self, d: &JetDirection) -> Shapes;
    fn points(&self) -> Vec<Point>;
}

impl Shape for Shapes {
    fn covers(&self, c: &Point) -> bool {
        match self {
            Shapes::HLine(p) => p.y == c.y && c.x >= p.x - 1 && c.x <= p.x + 2,
            Shapes::Cross(p) => p.dist(c) <= 1,
            Shapes::Ell(p) => {
                (p.y == c.y && c.x <= p.x && p.dist(c) <= 2)
                    || (p.x == c.x && c.y >= p.y && p.dist(c) <= 2)
            }
            Shapes::VLine(p) => p.x == c.x && c.y >= p.y - 1 && c.y <= p.y + 2,
            Shapes::Square(p) => c.x <= p.x && c.y <= p.y && p.dist(c) <= 1,
        }
    }

    fn pos(&self) -> &Point {
        match self {
            Shapes::HLine(p) => &p,
            Shapes::Cross(p) => &p,
            Shapes::Ell(p) => &p,
            Shapes::VLine(p) => &p,
            Shapes::Square(p) => &p,
        }
    }

    fn bounds(&self) -> (Point, Point) {
        match self {
            Shapes::HLine(p) => (Point::new(p.x - 1, p.y), Point::new(p.x + 2, p.y)),
            Shapes::Cross(p) => (Point::new(p.x - 1, p.y - 1), Point::new(p.x + 1, p.y + 1)),
            Shapes::Ell(p) => (Point::new(p.x - 2, p.y - 2), Point::new(p.x, p.y)),
            Shapes::VLine(p) => (Point::new(p.x, p.y - 1), Point::new(p.x, p.y + 2)),
            Shapes::Square(p) => (Point::new(p.x - 1, p.y - 1), Point::new(p.x, p.y)),
        }
    }

    fn touches(&self, l: &Line) -> bool {
        let (pmin, pmax) = self.bounds();
        l.on_infinite(&pmin) || l.on_infinite(&pmax)
    }

    fn dropped(&self) -> Shapes {
        let newpos = *self.pos() + Point::new(0, -1);
        match self {
            Shapes::HLine(_) => Shapes::HLine(newpos),
            Shapes::Cross(_) => Shapes::Cross(newpos),
            Shapes::Ell(_) => Shapes::Ell(newpos),
            Shapes::VLine(_) => Shapes::VLine(newpos),
            Shapes::Square(_) => Shapes::Square(newpos),
        }
    }

    fn pushed(&self, d: &JetDirection) -> Shapes {
        let newpos = match d {
            JetDirection::Left => *self.pos() - Point::new(1, 0),
            JetDirection::Right => *self.pos() + Point::new(1, 0),
        };
        match self {
            Shapes::HLine(_) => Shapes::HLine(newpos),
            Shapes::Cross(_) => Shapes::Cross(newpos),
            Shapes::Ell(_) => Shapes::Ell(newpos),
            Shapes::VLine(_) => Shapes::VLine(newpos),
            Shapes::Square(_) => Shapes::Square(newpos),
        }
    }

    fn points(&self) -> Vec<Point> {
        match self {
            Shapes::HLine(p) => vec![
                Point::new(p.x - 1, p.y),
                Point::new(p.x, p.y),
                Point::new(p.x + 1, p.y),
                Point::new(p.x + 2, p.y),
            ],
            Shapes::Cross(p) => vec![
                Point::new(p.x - 1, p.y),
                Point::new(p.x, p.y),
                Point::new(p.x + 1, p.y),
                Point::new(p.x, p.y + 1),
                Point::new(p.x, p.y - 1),
            ],
            Shapes::Ell(p) => vec![
                Point::new(p.x - 2, p.y),
                Point::new(p.x - 1, p.y),
                Point::new(p.x, p.y),
                Point::new(p.x, p.y + 1),
                Point::new(p.x, p.y + 2),
            ],
            Shapes::VLine(p) => vec![
                Point::new(p.x, p.y - 1),
                Point::new(p.x, p.y),
                Point::new(p.x, p.y + 1),
                Point::new(p.x, p.y + 2),
            ],
            Shapes::Square(p) => vec![
                Point::new(p.x - 1, p.y + 1),
                Point::new(p.x - 1, p.y),
                Point::new(p.x, p.y + 1),
                Point::new(p.x, p.y),
            ],
        }
    }
}

fn parse_jet_pattern(input: &str) -> Vec<JetDirection> {
    let mut result = Vec::new();
    for c in input.chars().filter(|c| *c != '\n') {
        result.push(match c {
            '>' => JetDirection::Right,
            '<' => JetDirection::Left,
            _ => panic!("Unknown jet direction {}", c),
        });
    }
    result
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum MapState {
    Solid,
    Free,
}

impl Default for MapState {
    fn default() -> Self {
        MapState::Free
    }
}

fn solution1(input: &str, rounds: u32) -> u32 {
    let jet_pattern = parse_jet_pattern(input);
    let mut jets = jet_pattern.into_iter().cycle();
    let mut map = SparseMap::new();
    let (left, right) = (
        Line::new(&Point::new(0, 0), &Point::new(0, 10)),
        Line::new(&Point::new(8, 0), &Point::new(8, 10)),
    );
    for i in 1..=7 {
        map.put(&Point::new(i, 0), MapState::Solid);
    }
    for i in 1..=rounds {
        let highest_rock = map.bounds().1.y;
        //println!("Highest rock: {}", highest_rock);
        let mut new_shape = match (i - 1) % 5 {
            0 => Shapes::HLine(Point::new(4, highest_rock + 4)),
            1 => Shapes::Cross(Point::new(4, highest_rock + 5)),
            2 => Shapes::Ell(Point::new(5, highest_rock + 4)),
            3 => Shapes::VLine(Point::new(3, highest_rock + 5)),
            4 => Shapes::Square(Point::new(4, highest_rock + 4)),
            _ => panic!("Unknown rock shape for round {}", i),
        };
        loop {
            let direction = jets.next().unwrap();
            let pushed = new_shape.pushed(&direction);
            if !pushed.touches(&left)
                && !pushed.touches(&right)
                && !pushed.points().iter().any(|p| map.not_empty(p))
            {
                if pushed.bounds().1.x > 7 {
                    println!("{:?}", pushed.points());
                    panic!("Tried to push shape {:?} outside of boundaries", pushed);
                }
                new_shape = pushed;
            }
            let dropped = new_shape.dropped();
            if dropped.points().iter().any(|p| map.not_empty(p)) {
                break;
            }
            new_shape = dropped;
        }
        for p in new_shape.points() {
            map.put(&p, MapState::Solid);
        }
    }
    map.bounds().1.y.try_into().unwrap()
}

#[ignore]
fn solution2(_input: &str) -> isize {
    0
}

#[cfg(test)]
mod test {
    use crate::{solution1, solution2};

    const TEST_STRING: &str = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(TEST_STRING, 2022), 3068);
    }

    #[ignore]
    #[test]
    fn test_solution2() {
        assert_eq!(solution2(TEST_STRING), 56000011);
    }
}

fn main() {
    println!(
        "Solution 1: {}",
        solution1(&read_to_string("input.txt").unwrap(), 2022)
    );
    // println!(
    //     "Solution 2: {}",
    //     solution2(&read_to_string("input.txt").unwrap())
    // );
}
