#![allow(dead_code, unused_variables, unused_imports)]
extern crate nom;

use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use rayon::prelude::*;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::Range,
};
use utils::{Point, SparseMap};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct BeaconInfo {
    sensor_pos: Point,
    beacon_pos: Point,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct NegativeBeaconInfo {
    sensor_pos: Point,
    dist: u32,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum MapState {
    Empty,
    Beacon,
    Sensor,
}

impl Default for MapState {
    fn default() -> Self {
        MapState::Empty
    }
}

struct BeaconMap {
    map: SparseMap<MapState>,
}

impl BeaconMap {
    fn new() -> Self {
        Self {
            map: SparseMap::new(),
        }
    }

    fn put_beacon_info(&mut self, b: &BeaconInfo) {
        self.map.put(&b.sensor_pos, MapState::Sensor);
        self.map.put(&b.beacon_pos, MapState::Beacon);
    }

    fn x_bounds(&self) -> (isize, isize) {
        let (pmin, pmax) = self.map.bounds();
        (pmin.x, pmax.x)
    }

    fn at(&self, p: &Point) -> MapState {
        self.map.at(p)
    }

    fn is_empty(&self, p: &Point) -> bool {
        self.map.is_empty(p)
    }
}

impl BeaconInfo {
    fn new(sensor_x: isize, sensor_y: isize, beacon_x: isize, beacon_y: isize) -> Self {
        Self {
            sensor_pos: Point::new(sensor_x, sensor_y),
            beacon_pos: Point::new(beacon_x, beacon_y),
        }
    }
}

fn parse_beacon_info(input: &str) -> IResult<&str, BeaconInfo> {
    map(
        tuple((
            preceded(tag("Sensor at x="), i32),
            preceded(tag(", y="), i32),
            preceded(tag(": closest beacon is at x="), i32),
            preceded(tag(", y="), i32),
        )),
        |(sensor_x, sensor_y, beacon_x, beacon_y)| {
            BeaconInfo::new(
                sensor_x.try_into().unwrap(),
                sensor_y.try_into().unwrap(),
                beacon_x.try_into().unwrap(),
                beacon_y.try_into().unwrap(),
            )
        },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<BeaconInfo>> {
    separated_list1(newline, parse_beacon_info)(input)
}

fn solution1(input: &str, row: i32) -> u32 {
    let all_info = parse_input(input).unwrap().1;
    let mut beacon_map = BeaconMap::new();
    let mut negative_info = Vec::new();
    for info in all_info {
        beacon_map.put_beacon_info(&info);
        negative_info.push(NegativeBeaconInfo {
            sensor_pos: info.sensor_pos,
            dist: info.sensor_pos.dist(&info.beacon_pos),
        });
    }
    let (min_x, max_x) = beacon_map.x_bounds();
    let mut result = 0;
    let max_dist = negative_info.iter().map(|n| n.dist).max().unwrap() as isize;
    let (start_point, end_point) = (
        Point::new(min_x - max_dist, row as isize),
        Point::new(max_x + max_dist, row as isize),
    );
    for tocheck in start_point.orthogonal_range(&end_point) {
        for no_beacon in &negative_info {
            if tocheck.dist(&no_beacon.sensor_pos) <= no_beacon.dist
                && beacon_map.at(&tocheck) != MapState::Beacon
            {
                result += 1;
                break;
            }
        }
    }
    result
}

fn permissible(ni: &NegativeBeaconInfo, tocheck: &Point) -> bool {
    ni.sensor_pos.dist(tocheck) > ni.dist
}

fn solution2(input: &str, bounds: isize) -> isize {
    let all_info = parse_input(input).unwrap().1;
    let mut negative_info: Vec<NegativeBeaconInfo> = Vec::new();
    let mut beacon_map = BeaconMap::new();
    for bi in all_info {
        beacon_map.put_beacon_info(&bi);
        negative_info.push(NegativeBeaconInfo {
            sensor_pos: bi.sensor_pos,
            dist: bi.sensor_pos.dist(&bi.beacon_pos),
        });
    }
    let possible_points: HashSet<Point> = negative_info.par_iter().flat_map(|ni| {
                ni.sensor_pos.points_at_dist((ni.dist + 1).try_into().unwrap())
                .filter(|p| p.x >= 0 && p.x <= bounds && p.y >= 0 && p.y <= bounds).collect::<HashSet<Point>>()
    }).collect();
    let result = possible_points.par_iter().find_any(|p| {
        negative_info.iter().all(|ni| ni.sensor_pos.dist(p) > ni.dist)
    }).expect("No result found.");
    return result.x * 4000000 + result.y;
}

#[cfg(test)]
mod test {
    use utils::Point;

    use crate::{parse_beacon_info, parse_input, solution1, solution2, BeaconInfo};

    const TEST_STRING: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_parse_beacon_info() {
        let (_, result) =
            parse_beacon_info("Sensor at x=2, y=18: closest beacon is at x=-2, y=15").unwrap();
        assert_eq!(result, BeaconInfo::new(2, 18, -2, 15));
    }

    #[test]
    fn test_parse_input() {
        let (_, result) = parse_input(TEST_STRING).unwrap();
        assert_eq!(
            result,
            vec![
                BeaconInfo::new(2, 18, -2, 15),
                BeaconInfo::new(9, 16, 10, 16),
                BeaconInfo::new(13, 2, 15, 3),
                BeaconInfo::new(12, 14, 10, 16),
                BeaconInfo::new(10, 20, 10, 16),
                BeaconInfo::new(14, 17, 10, 16),
                BeaconInfo::new(8, 7, 2, 10),
                BeaconInfo::new(2, 0, 2, 10),
                BeaconInfo::new(0, 11, 2, 10),
                BeaconInfo::new(20, 14, 25, 17),
                BeaconInfo::new(17, 20, 21, 22),
                BeaconInfo::new(16, 7, 15, 3),
                BeaconInfo::new(14, 3, 15, 3),
                BeaconInfo::new(20, 1, 15, 3)
            ]
        );
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(TEST_STRING, 10), 26);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(TEST_STRING, 20), 56000011);
    }
}

fn main() {
    println!(
        "Solution 1: {}",
        solution1(&read_to_string("input.txt").unwrap(), 2000000)
    );
    println!(
        "Solution 2: {}",
        solution2(&read_to_string("input.txt").unwrap(), 4000000)
    );
}
