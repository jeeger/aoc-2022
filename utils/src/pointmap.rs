use std::collections::HashSet;

use crate::point::Point;

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
            LookDirection::ToWest,
        ];
        ALL_DIRECTIONS
    }
}

pub type PointMap = Vec<Vec<u32>>;
pub trait PointMappable {
    fn load(l: impl Iterator<Item = String>) -> Self;
    fn load_with(l: impl Iterator<Item = String>, transform: Box<dyn Fn(char) -> u32>) -> Self;
    fn at(&self, p: &Point) -> u32;
    fn sizex(&self) -> usize;
    fn sizey(&self) -> usize;
    fn points_in_direction(&self, p: &Point, d: &LookDirection) -> Vec<Point>;
    fn adjacent_points(&self, p: &Point) -> HashSet<Point>;
    fn all_points(&self) -> Box<dyn Iterator<Item = Point> + '_>;
    fn print(&self);
}

impl PointMappable for PointMap {
    fn load(l: impl Iterator<Item = String>) -> Self {
        Self::load_with(l, Box::new(|c: char| c.to_digit(10).unwrap()))
    }

    fn load_with(l: impl Iterator<Item = String>, transform: Box<dyn Fn(char) -> u32>) -> Self {
        let mut data: Vec<Vec<u32>> = vec![];
        for l in l {
            data.push(l.chars().map(&transform).collect::<Vec<u32>>());
        }
        data
    }
    fn at(&self, p: &Point) -> u32 {
        if p.y < 0 || p.x < 0 {
            panic!("Map not implemented for negative points");
        }
        self[p.y as usize][p.x as usize]
    }

    fn sizex(&self) -> usize {
        self[0].len()
    }

    fn sizey(&self) -> usize {
        self.len()
    }

    fn adjacent_points(&self, p: &Point) -> HashSet<Point> {
        let mut result = HashSet::new();
        for (xoffset, yoffset) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let new_point = *p + Point::new(xoffset, yoffset);
            if new_point.x >= 0
                && new_point.x < (self.sizex().try_into().unwrap())
                && new_point.y >= 0
                && new_point.y < (self.sizey().try_into().unwrap())
            {
                result.insert(new_point);
            }
        }
        result
    }

    fn points_in_direction(&self, p: &Point, d: &LookDirection) -> Vec<Point> {
        match *d {
            LookDirection::ToNorth => (0..p.y).map(|y| Point::new(p.x, y)).collect(),
            LookDirection::ToEast => (p.x + 1..self.sizex() as isize)
                .map(|x| Point::new(x as isize, p.y))
                .collect(),
            LookDirection::ToSouth => (p.y + 1..self.sizey() as isize)
                .map(|y| Point::new(p.x, y))
                .collect(),
            LookDirection::ToWest => (0..p.x).map(|x| Point::new(x, p.y)).collect(),
        }
    }

    fn all_points(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        Box::new(
            (0..self.sizex())
                .flat_map(|x| (0..self.sizey()).map(move |y| Point::new(x as isize, y as isize))),
        )
    }

    fn print(&self) {
        println!(
            "{}",
            self.iter()
                .map(|r| r.iter().map(|c| format!("{:3}", c)).collect::<Vec<String>>().join(""))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}
