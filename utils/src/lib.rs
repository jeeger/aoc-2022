use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;

pub fn lines(filename: &str) -> impl Iterator<Item = String> {
    let r = BufReader::new(File::open(filename).expect("Failed to open file"));
    r.lines().map(|elem| elem.expect("Couldn't read line"))
}

pub fn split_lines(filename: &str) -> impl Iterator<Item = Vec<String>> {
    lines(filename)
        .filter(|s| !s.is_empty())
        .map(|s| s.split_whitespace().map(|str| String::from(str)).collect())
}

pub fn split_lines_sep(filename: &str, sep: char) -> impl Iterator<Item = Vec<String>> {
    lines(filename)
        .filter(|s| !s.is_empty())
        .map(move |s| s.split(sep).map(|str| String::from(str)).collect())
}

pub fn char_lines(filename: &str) -> impl Iterator<Item = Vec<char>> {
    lines(filename)
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
}

pub fn string_lines(s: &str) -> impl Iterator<Item = String> + '_ {
    let r = BufReader::new(Cursor::new(s)).lines();
    r.map(|elem| elem.expect("Couldn't read line"))
}

pub fn string_split_lines(s: &str) -> impl Iterator<Item = Vec<String>> + '_ {
    println!("{}", s);
    string_lines(s)
        .filter(|s| !s.is_empty())
        .map(|s| s.split_whitespace().map(|f| String::from(f)).collect())
}

pub fn string_char_lines(s: &str) -> impl Iterator<Item = Vec<char>> + '_ {
    string_lines(s)
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
}

pub fn string_split_lines_sep(s: &str, sep: char) -> impl Iterator<Item = Vec<String>> + '_ {
    string_lines(s)
        .filter(|s| !s.is_empty())
        .map(move |s| s.split(sep).map(|str| String::from(str)).collect())
}

pub fn str_between<'a>(line: &'a str, left: Option<&str>, right: Option<&str>) -> &'a str {
    let left_offset = left
        .map(|s| line.find(s).expect("Could not find left separator") + s.len())
        .unwrap_or(0);
    let right_offset = right
        .map(|s| line.find(s).expect("Could not find right separator"))
        .unwrap_or(line.len());
    &line[left_offset..right_offset]
}

pub fn num_between(line: &str, left: Option<&str>, right: Option<&str>) -> u32 {
    str_between(line, left, right).parse::<u32>().expect(
        format!(
            "Could not parse num between '{:?}' and '{:?}' in line {}",
            left, right, line
        )
        .as_str(),
    )
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LookDirection {
    ToNorth,
    ToEast,
    ToSouth,
    ToWest,
}

impl LookDirection {
    pub fn all() -> [LookDirection; 4] {
        static ALL_DIRECTIONS: [LookDirection; 4] = [
            LookDirection::ToNorth,
            LookDirection::ToEast,
            LookDirection::ToSouth,
            LookDirection::ToWest
        ];
        ALL_DIRECTIONS
    }
}

pub type PointMap = Vec<Vec<u32>>;
pub trait PointMappable {
    fn load(l: impl Iterator<Item = String>) -> Self;
    fn at(&self, p: &Point) -> u32;
    fn sizex(&self) -> usize;
    fn sizey(&self) -> usize;
    fn points_in_direction(&self, p: &Point, d: &LookDirection) -> Vec<Point>;
    fn all_points(&self) -> Box<dyn Iterator<Item = Point> + '_>;
}

impl PointMappable for PointMap {
    fn load(l: impl Iterator<Item = String>) -> Self {
        let mut data: Vec<Vec<u32>> = vec![];
        for l in l {
            data.push(
                l.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>(),
            );
        }
        data
    }
    fn at(&self, p: &Point) -> u32 {
        self[p.y][p.x]
    }

    fn sizex(&self) -> usize {
        self[0].len()
    }

    fn sizey(&self) -> usize {
        self.len()
    }

    fn points_in_direction(&self, p: &Point, d: &LookDirection) -> Vec<Point> {
        match *d {
            LookDirection::ToNorth => (0..p.y).map(|y| Point::new(p.x, y)).collect(),
            LookDirection::ToEast => (p.x + 1..self.sizex())
                .map(|x| Point::new(x, p.y))
                .collect(),
            LookDirection::ToSouth => (p.y + 1..self.sizey())
                .map(|y| Point::new(p.x, y))
                .collect(),
            LookDirection::ToWest => (0..p.x).map(|x| Point::new(x, p.y)).collect(),
        }
    }

    fn all_points(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        Box::new((0..self.sizex()).flat_map(|x| (0..self.sizey()).map(move |y| Point::new(x, y))))
    }
}

#[cfg(test)]
mod test {
    use crate::{num_between, string_lines, PointMappable, PointMap, Point, LookDirection};

    #[test]
    fn test_num_between_simple() {
        let result = num_between("move 1 from 2 to 3", Some("move "), Some(" from "));
        assert_eq!(result, 1);
    }

    #[test]
    fn test_num_between_simple_2() {
        let result = num_between("move 1 from 2 to 3", Some("from "), Some(" to "));
        assert_eq!(result, 2);
    }

    #[test]
    fn test_num_between_to_end() {
        let result = num_between("move 1 from 2 to 3", Some(" to "), None);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_map_construction() {
        let m: PointMap = PointMappable::load(string_lines(r"30373
25512
65332
33549
35390
"));
        assert_eq!(m.sizex(), 5);
        assert_eq!(m.sizey(), 5);
        assert_eq!(m, vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]
        ]);
    }

    #[test]
    fn test_map_indexing() {
        let m = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]];
        assert_eq!(m.at(&Point::new(0, 0)), 3);
        assert_eq!(m.at(&Point::new(3, 3)), 4);
    }

    #[test]
    fn test_points_in_direction() {
        let m = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]];
        let points_north = m.points_in_direction(&Point::new(2, 2), &LookDirection::ToNorth);
        let points_east = m.points_in_direction(&Point::new(2, 2), &LookDirection::ToEast);
        let points_south = m.points_in_direction(&Point::new(2, 2), &LookDirection::ToSouth);
        let points_west = m.points_in_direction(&Point::new(2, 2), &LookDirection::ToWest);
        assert_eq!(points_north, vec![Point::new(2, 0), Point::new(2, 1)]);
        assert_eq!(points_east, vec![Point::new(3, 2), Point::new(4, 2)]);
        assert_eq!(points_south, vec![Point::new(2, 3), Point::new(2, 4)]);
        assert_eq!(points_west, vec![Point::new(0, 2), Point::new(1, 2)]);
    }

    #[test]
    fn test_all_points() {
        let m: PointMap = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]];
        assert_eq!(m.all_points().count(), 25);
    }
}
