#![feature(iter_array_chunks)]
use std::collections::HashSet;
use utils::char_lines;
use utils::lines;

fn char_priority(c: &char) -> u32 {
    if c.is_uppercase() {
        *c as u32 - 64 + 26
    } else {
        *c as u32 - 96
    }
}

fn score_compartments_single(l: &HashSet<char>, r: &HashSet<char>) -> u32 {
    l.intersection(r).map(char_priority).sum()
}

fn solution1(lines: impl Iterator<Item = Vec<char>>) -> u32 {
    let mut result = 0;
    for mut line in lines {
        let compartment_size = line.len() / 2;
        let second_compartment = HashSet::from_iter(line.split_off(compartment_size));
        let first_compartment = HashSet::from_iter(line);
        result += score_compartments_single(&first_compartment, &second_compartment);
    }
    result
}

fn solution2(lines: impl Iterator<Item = String>) -> u32 {
    let mut result = 0;
    for [group1, group2, group3] in lines.array_chunks() {
        let g1: HashSet<char> = HashSet::from_iter(group1.chars());
        let g2 = HashSet::from_iter(group2.chars());
        let g3 = HashSet::from_iter(group3.chars());
        let mut common: HashSet<_> = g1.intersection(&g2).cloned().collect();
        common = common.intersection(&g3).cloned().collect();
        for c in common {
            result += char_priority(&c);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::{solution1, solution2};
    use utils::string_char_lines;

    fn test_iter() -> impl Iterator<Item = Vec<char>> {
        string_char_lines(
            r"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
",
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(test_iter()), 157);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(test_iter().map(|vec| String::from_iter(vec))), 70);
    }
}

fn main() {
    println!("Solution 1: {}", solution1(char_lines("input.txt")));
    println!("Solution 2: {}", solution2(lines("input.txt")));
}
