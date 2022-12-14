#![allow(dead_code, unused_variables)]
extern crate nom;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::read_to_string,
};
use utils::Point;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SpaceType {
    Sand,
    Rock,
    Empty,
}

struct GroundMap {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    map: HashMap<Point, SpaceType>,
}

fn point_range<'a>(p1: &'a Point, p2: &'a Point) -> Box<dyn Iterator<Item = Point> + 'a> {
    if p1.x != p2.x && p1.y != p2.y {
        panic!("Can only generate orthogonal ranges!");
    };
    if p1.x == p2.x {
        Box::new((min(p1.y, p2.y)..max(p1.y, p2.y) + 1).map(|y| Point::new(p1.x, y)))
    } else {
        Box::new((min(p1.x, p2.x)..max(p1.x, p2.x) + 1).map(|x| Point::new(x, p1.y)))
    }
}

impl GroundMap {
    fn new() -> Self {
        GroundMap {
            min_x: isize::MAX,
            max_x: isize::MIN,
            min_y: isize::MAX,
            max_y: isize::MIN,
            map: HashMap::new(),
        }
    }

    fn at(&self, p: &Point) -> SpaceType {
        *self.map.get(p).unwrap_or(&SpaceType::Empty)
    }

    fn bounds(&self) -> (Point, Point) {
        (
            Point::new(self.min_x, self.min_y),
            Point::new(self.max_x, self.max_y),
        )
    }

    fn put_rock(&mut self, p: &Point) {
        self.min_x = min(p.x, self.min_x);
        self.max_x = max(p.x, self.max_x);
        self.min_y = min(p.y, self.min_y);
        self.max_y = max(p.y, self.max_y);
        self.map.insert(*p, SpaceType::Rock);
    }

    fn add_rock_between(&mut self, p1: &Point, p2: &Point) {
        for path_point in point_range(p1, p2) {
            self.put_rock(&path_point);
        }
    }

    fn has_left(&self, p: &Point) -> bool {
        p.x < self.min_x || p.x > self.max_x || p.y > self.max_y
    }

    fn put_sand(&mut self, p: &Point) {
        // Only complain when putting sand outside the map borders to the left, right and bottom
        if self.has_left(p) {
            panic!(
                "Trying to put sand at {:?}, outside map bounds {},{}, {}, {}",
                p, self.min_x, self.min_y, self.max_x, self.max_y
            )
        }
        if self.is_solid(p) {
            panic!(
                "Trying to put sand at {:?}, where there is already {:?}",
                p,
                self.at(p)
            );
        }
        self.map.insert(*p, SpaceType::Sand);
    }

    fn is_solid(&self, p: &Point) -> bool {
        let t = *self.map.get(p).unwrap_or(&SpaceType::Empty);
        t == SpaceType::Rock || t == SpaceType::Sand
    }

    fn is_empty(&self, p: &Point) -> bool {
        !self.map.contains_key(p)
    }
}

fn parse_coord(i: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, tag(","), u32)(i)
}

fn parse_line(text: &str) -> IResult<&str, Vec<(u32, u32)>> {
    terminated(separated_list1(tag(" -> "), parse_coord), newline)(text)
}

fn parse_input(text: &str) -> IResult<&str, Vec<Vec<(u32, u32)>>> {
    many1(parse_line)(text)
}

fn construct_map(rock: Vec<Vec<(u32, u32)>>) -> GroundMap {
    let mut result = GroundMap::new();
    for path in rock {
        for i in 1..path.len() {
            result.add_rock_between(
                &Point::new(path[i - 1].0 as isize, path[i - 1].1 as isize),
                &Point::new(path[i].0 as isize, path[i].1 as isize),
            )
        }
    }
    result
}

fn construct_map_solution2(rock: Vec<Vec<(u32, u32)>>) -> GroundMap {
    let mut result = construct_map(rock);
    let floor_y = result.bounds().1.y + 2;
    result.add_rock_between(&Point::new(500 - floor_y, floor_y), &Point::new(500 + floor_y, floor_y));
    result
}

fn points_to_check(p: &Point) -> [Point; 3] {
    [
        *p + Point::new(0, 1),
        *p + Point::new(-1, 1),
        *p + Point::new(1, 1),
    ]
}

fn drop_sand(m: &GroundMap, drop_point: &Point) -> (Option<Point>, bool) {
    if m.is_solid(&drop_point) {
        return (None, true);
    }
    let mut current_point = *drop_point;
    loop {
        if m.has_left(&current_point) {
            return (None, false);
        }
        let [down, left, right] = points_to_check(&current_point);
        if m.is_empty(&down) {
            current_point = down;
        } else if m.is_empty(&left) {
            current_point = left;
        } else if m.is_empty(&right) {
            current_point = right;
        } else {
            return (Some(current_point), false);
        }
    }
}

fn solution1(input: &str) -> u32 {
    let (_, input) = parse_input(input).unwrap();
    let mut m = construct_map(input);
    let mut dropped = 0;
    while let (Some(p), _) = drop_sand(&m, &Point::new(500, 0)) {
        m.put_sand(&p);
        dropped += 1;
    }
    dropped
}

fn solution2(input: &str) -> u32 {
    let (_, input) = parse_input(input).unwrap();
    let mut m = construct_map_solution2(input);
    let mut dropped = 0;
    loop {
        let (p, blocked) = drop_sand(&m, &Point::new(500, 0));
        if blocked {
            return dropped;
        }
        if p.is_none() {
            panic!("Sand fell off before inlet was blocked");
        }
        m.put_sand(&p.unwrap());
        dropped += 1;
    }
}

#[cfg(test)]
mod test {
    use utils::Point;

    use crate::{construct_map, drop_sand, parse_input, parse_line, solution1, solution2, construct_map_solution2};

    const TEST_STRING: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_parse_line() {
        let test_string = "498,4 -> 498,6 -> 496,6\n";
        match parse_line(test_string) {
            Ok((rest, result)) => {
                assert_eq!(result, vec![(498, 4), (498, 6), (496, 6)]);
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_parse_input() {
        match parse_input(TEST_STRING) {
            Ok((rest, output)) => {
                assert!(rest.is_empty());
                assert_eq!(output.len(), 2);
                assert_eq!(output[0], vec![(498, 4), (498, 6), (496, 6)]);
                assert_eq!(output[1], vec![(503, 4), (502, 4), (502, 9), (494, 9)]);
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_map_construction() {
        let input = vec![
            vec![(498, 4), (498, 6), (496, 6)],
            vec![(503, 4), (502, 4), (502, 9), (494, 9)],
        ];
        let map = construct_map(input);
        assert!(map.is_solid(&Point::new(498, 4)));
        assert!(map.is_solid(&Point::new(495, 9)));
        assert!(!map.is_solid(&Point::new(494, 0)));
        assert_eq!(map.bounds(), (Point::new(494, 4), Point::new(503, 9)));
    }

    #[test]
    fn test_map_construction_solution2() {
        let input = vec![
            vec![(498, 4), (498, 6), (496, 6)],
            vec![(503, 4), (502, 4), (502, 9), (494, 9)],
        ];
        let map = construct_map_solution2(input);
        assert!(map.is_solid(&Point::new(500, 11)));
        assert!(map.is_solid(&Point::new(489, 11)));
        assert!(map.is_solid(&Point::new(511, 11)));
        assert!(!map.is_solid(&Point::new(488, 11)));
        assert!(!map.is_solid(&Point::new(512, 11)));
    }

    #[test]
    fn test_sand_drop() {
        let (_, output) = parse_input(TEST_STRING).unwrap();
        let map = construct_map(output);
        assert_eq!(
            drop_sand(&map, &Point::new(500, 0)),
            (Some(Point::new(500, 8)), false)
        );
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(TEST_STRING), 24);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(TEST_STRING), 93);
    }
}

fn main() {
    println!(
        "Solution 1: {}",
        solution1(&read_to_string("input.txt").unwrap())
    );
    println!(
        "Solution 2: {}",
        solution2(&read_to_string("input.txt").unwrap())
    );
}
