use utils::{lines, LookDirection, Point, PointMap, PointMappable};

fn visible_from(data: &PointMap, p: &Point, look_direction: &LookDirection) -> bool {
    let point_height = data.at(p);
    let points_to_look_at = data.points_in_direction(p, look_direction);
    points_to_look_at.iter().all(|p| {
        let visible = data.at(p) < point_height;
        visible
    })
}

fn is_visible(data: &PointMap, p: &Point) -> bool {
    let sizex = data[0].len();
    let sizey = data.len();
    if p.x == 0 || p.y == 0 || p.x == (sizex - 1) || p.y == (sizey - 1) {
        return true;
    };
    for d in [
        LookDirection::ToNorth,
        LookDirection::ToEast,
        LookDirection::ToSouth,
        LookDirection::ToWest,
    ] {
        if visible_from(data, p, &d) {
            return true;
        }
    }
    false
}

fn trees_visible_from(map: &PointMap, p: &Point, d: &LookDirection) -> u32 {
    let mut points_to_consider = map.points_in_direction(p, d);
    if *d == LookDirection::ToNorth || *d == LookDirection::ToWest {
        points_to_consider.reverse();
    };
    let mut result = 0;
    for considered_point in points_to_consider {
        result += 1;
        if map.at(&considered_point) >= map.at(p) {
            return result;
        }
    }
    return result;
}

fn scenic_score(map: &PointMap, p: &Point) -> u32 {
    LookDirection::all()
        .map(|d| trees_visible_from(map, p, &d))
        .iter()
        .cloned()
        .reduce(|prod, count| prod * count)
        .unwrap_or(0)
}

fn solution1(lines: impl Iterator<Item = String>) -> u32 {
    let mut result = 0;
    let data: PointMap = PointMappable::load(lines);
    for p in data.all_points() {
        if is_visible(&data, &p) {
            result += 1;
        }
    }
    result
}

fn solution2(lines: impl Iterator<Item = String>) -> u32 {
    let data: PointMap = PointMappable::load(lines);
    data.all_points().map(|p| scenic_score(&data, &p)).max().expect("No maximum value")
}

#[cfg(test)]
mod test {
    use crate::{is_visible, solution1, solution2, trees_visible_from, scenic_score};
    use utils::{string_lines, LookDirection, Point, PointMap, PointMappable};

    fn test_iter() -> impl Iterator<Item = String> {
        string_lines(
            r"30373
25512
65332
33549
35390",
        )
    }

    #[test]
    fn test_is_not_visible() {
        let map: PointMap = PointMappable::load(test_iter());
        assert!(!is_visible(&map, &Point { x: 3, y: 1 }));
    }

    #[test]
    fn trees_visible_from_test() {
        let map: PointMap = PointMappable::load(test_iter());
        assert_eq!(
            trees_visible_from(&map, &Point::new(2, 1), &LookDirection::ToNorth),
            1
        );
        assert_eq!(
            trees_visible_from(&map, &Point::new(2, 1), &LookDirection::ToWest),
            1
        );
        assert_eq!(
            trees_visible_from(&map, &Point::new(2, 1), &LookDirection::ToEast),
            2
        );
        assert_eq!(
            trees_visible_from(&map, &Point::new(2, 1), &LookDirection::ToSouth),
            2
        );
    }

    #[test]
    fn test_scenic_score() {
        let map: PointMap = PointMappable::load(test_iter());
        assert_eq!(scenic_score(&map, &Point::new(2, 1)), 4);
        assert_eq!(scenic_score(&map, &Point::new(2, 3)), 8);
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(test_iter()), 21);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(test_iter()), 8);
    }
}

fn main() {
    println!("Solution 1: {}", solution1(lines("input.txt")));
    println!("Solution 2: {}", solution2(lines("input.txt")));
}
