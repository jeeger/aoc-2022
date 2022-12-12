mod parsing;
mod point;
mod pointmap;
pub use parsing::*;
pub use point::Point;
pub use pointmap::{LookDirection, PointMap, PointMappable};

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{num_between, string_lines, LookDirection, Point, PointMap, PointMappable};

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
        let m: PointMap = PointMappable::load(string_lines(
            r"30373
25512
65332
33549
35390
",
        ));
        assert_eq!(m.sizex(), 5);
        assert_eq!(m.sizey(), 5);
        assert_eq!(
            m,
            vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0]
            ]
        );
    }

    #[test]
    fn test_map_indexing() {
        let m = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
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
            vec![3, 5, 3, 9, 0],
        ];
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
    fn test_adjacent_points() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            m.adjacent_points(&Point::new(1, 1)),
            HashSet::from([
                Point::new(1, 0),
                Point::new(1, 2),
                Point::new(0, 1),
                Point::new(2, 1)
            ])
        );
        assert_eq!(
            m.adjacent_points(&Point::new(0, 0)),
            HashSet::from([Point::new(1, 0), Point::new(0, 1)])
        );
    }

    #[test]
    fn test_all_points() {
        let m: PointMap = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(m.all_points().count(), 25);
    }

    #[test]
    fn test_adjacent() {
        assert!(Point::new(1, 0).is_adjacent(&Point::new(2, 0)));
        assert!(Point::new(1, 0).is_adjacent(&Point::new(1, 1)));
        assert!(Point::new(1, 0).is_adjacent(&Point::new(0, 0)));
        assert!(Point::new(1, 1).is_adjacent(&Point::new(1, 0)));
        assert!(!Point::new(1, 1).is_adjacent(&Point::new(3, 1)));
    }
}
