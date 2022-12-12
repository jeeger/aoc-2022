use std::collections::HashSet;
use std::{convert::Infallible, str::FromStr};

use utils::{lines, num_between, str_between, Point};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MoveDirection {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Step {
    count: u32,
    dir: MoveDirection,
}

impl Step {
    fn new(dir: MoveDirection, count: u32) -> Self {
        Step { count, dir }
    }
}

impl FromStr for Step {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count = num_between(s, Some(" "), None);
        let direction = match str_between(s, None, Some(" ")) {
            "R" => MoveDirection::Right,
            "U" => MoveDirection::Up,
            "D" => MoveDirection::Down,
            "L" => MoveDirection::Left,
            _ => panic!("Unknown direction."),
        };
        Ok(Step::new(direction, count.try_into().unwrap()))
    }
}

fn parse_steps(lines: impl Iterator<Item = String>) -> Vec<Step> {
    lines
        .map(|l| l.parse().expect("Failed to parse step"))
        .collect()
}

fn one_closer_to_zero(v: i32) -> i32 {
    if v == 0 {
        0
    } else if v < 0 {
        v + 1
    } else if v > 0 {
        v - 1
    } else {
        panic!("Unknown value")
    }
}

fn adjust(h_pos: &Point, mut t_pos: Point) -> Point {
       // Already adjacent, no adjustment needed.
    if h_pos.adjacent(&t_pos) {
        return t_pos;
    };
    let (xdist, ydist) = h_pos.vec_dist(&t_pos);
    // Happy orthogonal case
    if xdist == 0 || ydist == 0 {
        t_pos.adjust(one_closer_to_zero(xdist), one_closer_to_zero(ydist));
        t_pos
    } else {
        match (xdist, ydist) {
            (2, 1) | (2, 2) => t_pos.adjust(1, 1),
            (-2, 1) | (-2, 2) => t_pos.adjust(-1, 1),
            (1, 2) => t_pos.adjust(1, 1),
            (1, -2) | (2, -2) => t_pos.adjust(1, -1),
            (2, -1) => t_pos.adjust(1, -1),
            (-2, -1) | (-2, -2) => t_pos.adjust(-1, -1),
            (-1, 2) => t_pos.adjust(-1, 1),
            (-1, -2) => t_pos.adjust(-1, -1),
            _ => {
                println!("H: {:?} T: {:?}", h_pos, t_pos);
                panic!("Unknown distance combination {}:{}", xdist, ydist);
            }
            
        };
        t_pos
    }
}

fn single_step(d: &MoveDirection, mut h_pos: Point, t_pos: Point) -> (Point, Point) {
    if !h_pos.adjacent(&t_pos) {
        panic!("Head and tail are not adjacent before step.");
    };
    match d {
        MoveDirection::Up => h_pos += Point::new(0, 1),
        MoveDirection::Right => h_pos += Point::new(1, 0),
        MoveDirection::Left => h_pos -= Point::new(1, 0),
        MoveDirection::Down => h_pos -= Point::new(0, 1),
    };
    (h_pos, adjust(&h_pos, t_pos))
}

fn step_rope(d: &MoveDirection, mut h_pos: Point, mut tails: Vec<Point>) -> (Point, Vec<Point>) {
    if !h_pos.adjacent(&tails[0]) {
        panic!("Head and first tail are not adjacent before step.");
    };
    match d {
        MoveDirection::Up => h_pos += Point::new(0, 1),
        MoveDirection::Right => h_pos += Point::new(1, 0),
        MoveDirection::Left => h_pos -= Point::new(1, 0),
        MoveDirection::Down => h_pos -= Point::new(0, 1),
    };
    tails[0] = adjust(&h_pos, tails[0]);
    for i in 1..tails.len() {
        tails[i] = adjust(&tails[i - 1], tails[i]);
    }
    (h_pos, tails)
}

fn solution1(lines: impl Iterator<Item = String>) -> u32 {
    let mut result_set = HashSet::new();
    let mut h_pos = Point::new(0, 0);
    let mut t_pos = Point::new(0, 0);
    result_set.insert(t_pos);
    for step in parse_steps(lines) {
        for _i in 0..step.count {
            (h_pos, t_pos) = single_step(&step.dir, h_pos, t_pos);
            result_set.insert(t_pos);
        }
    };
    result_set.len() as u32
}

fn solution2(lines: impl Iterator<Item = String>) -> u32 {
    let mut result_set = HashSet::new();
    let mut h_pos = Point::new(0, 0);
    let mut tails: Vec<Point> = std::iter::repeat_with(|| {
        Point::new(0, 0)
    }).take(9).collect();
    for step in parse_steps(lines) {
        for _i in 0..step.count {
            let (new_h_pos, new_tails) = step_rope(&step.dir, h_pos, tails);
            result_set.insert(new_tails.last().copied().expect("No last element!"));
            h_pos = new_h_pos;
            tails = new_tails;
            
        };
    };
    result_set.len() as u32
}

#[cfg(test)]
mod test {
    use crate::{parse_steps, single_step, solution1, solution2, MoveDirection, Step};
    use utils::{string_lines, Point};

    fn test_iter() -> impl Iterator<Item = String> {
        string_lines(
            r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
",
        )
    }

    #[test]
    fn test_single_step_right() {
        let h_pos = Point::new(1, 0);
        let t_pos = Point::new(0, 0);
        let (new_h_pos, new_t_pos) = single_step(&MoveDirection::Right, h_pos, t_pos);
        assert_eq!(new_h_pos, Point::new(2, 0));
        assert_eq!(new_t_pos, Point::new(1, 0));
    }

    #[test]
    fn test_single_step_left() {
        let h_pos = Point::new(2, 0);
        let t_pos = Point::new(3, 0);
        let (new_h_pos, new_t_pos) = single_step(&MoveDirection::Left, h_pos, t_pos);
        assert_eq!(new_h_pos, Point::new(1, 0));
        assert_eq!(new_t_pos, Point::new(2, 0));
    }

    #[test]
    fn test_single_step_diagonal() {
        let h_pos = Point::new(4, 1);
        let t_pos = Point::new(3, 0);
        let (new_h_pos, new_t_pos) = single_step(&MoveDirection::Up, h_pos, t_pos);
        assert_eq!(new_h_pos, Point::new(4, 2));
        assert_eq!(new_t_pos, Point::new(4, 1));
    }

    #[test]
    fn parse_steps_test() {
        let parsed = parse_steps(test_iter());
        assert_eq!(
            parsed,
            vec![
                Step::new(MoveDirection::Right, 4),
                Step::new(MoveDirection::Up, 4),
                Step::new(MoveDirection::Left, 3),
                Step::new(MoveDirection::Down, 1),
                Step::new(MoveDirection::Right, 4),
                Step::new(MoveDirection::Down, 1),
                Step::new(MoveDirection::Left, 5),
                Step::new(MoveDirection::Right, 2)
            ]
        );
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(test_iter()), 13);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(test_iter()), 1);
    }
}

fn main() {
    println!("Solution 1: {}", solution1(lines("input.txt")));
    println!("Solution 2: {}", solution2(lines("input.txt")));
}
