#![feature(iter_array_chunks)]
use std::ops::Range;
use utils::split_lines_sep;

// Whether r1 is completely contained in r2 or the reverse
fn ranges_contained(r1: Range<u32>, r2: Range<u32>) -> bool {
    (r2.contains(&r1.start) && r2.contains(&(r1.end - 1)))
        || (r1.contains(&r2.start) && r1.contains(&(r2.end - 1)))
}

fn ranges_overlap(r1: Range<u32>, r2: Range<u32>) -> bool {
    r2.contains(&r1.start)
        || r2.contains(&(r1.end - 1))
        || r1.contains(&r2.start)
        || r1.contains(&(r2.end - 1))
}

fn solution1(lines: impl Iterator<Item = Vec<String>>) -> u32 {
    let mut result = 0;
    for pair in lines {
        let first_bounds: Vec<&str> = pair[0].split('-').collect();
        let second_bounds: Vec<&str> = pair[1].split('-').collect();
        let r1: Range<u32> = Range {
            start: first_bounds[0].parse().unwrap(),
            end: first_bounds[1].parse::<u32>().unwrap() + 1,
        };
        let r2: Range<u32> = Range {
            start: second_bounds[0].parse().unwrap(),
            end: second_bounds[1].parse::<u32>().unwrap() + 1,
        };
        if ranges_contained(r1, r2) {
            result += 1;
        }
    }
    result
}

fn solution2(lines: impl Iterator<Item = Vec<String>>) -> u32 {
    let mut result = 0;
    for pair in lines {
        let first_bounds: Vec<&str> = pair[0].split('-').collect();
        let second_bounds: Vec<&str> = pair[1].split('-').collect();
        let r1: Range<u32> = Range {
            start: first_bounds[0].parse().unwrap(),
            end: first_bounds[1].parse::<u32>().unwrap() + 1,
        };
        let r2: Range<u32> = Range {
            start: second_bounds[0].parse().unwrap(),
            end: second_bounds[1].parse::<u32>().unwrap() + 1,
        };
        if ranges_overlap(r1, r2) {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::{solution1, solution2};
    use utils::string_split_lines_sep;

    fn test_iter() -> impl Iterator<Item = Vec<String>> {
        string_split_lines_sep(
            r"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
",
            ',',
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(test_iter()), 2);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(test_iter()), 4);
    }
}

fn main() {
    println!(
        "Solution 1: {}",
        solution1(split_lines_sep("input.txt", ','))
    );
    println!(
        "Solution 2: {}",
        solution2(split_lines_sep("input.txt", ','))
    );
}
