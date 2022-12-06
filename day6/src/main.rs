#![feature(iter_array_chunks)]
#![feature(map_many_mut)]
use std::collections::HashSet;
use utils::lines;

struct PrefixIterator<'a> {
    s: &'a str,
    offset: usize,
    len: usize,
}

impl<'a> PrefixIterator<'a> {
    fn new(s: &'a str, len: usize) -> Self {
        PrefixIterator {
            offset: 0,
            len,
            s,
        }
    }
}

impl<'a> Iterator for PrefixIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset + self.len <= self.s.len() {
            let result = &self.s[self.offset..self.offset + self.len];
            self.offset += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn solution1(mut lines: impl Iterator<Item = String>) -> u32 {
    let string = lines.next().expect("Expected single line");
    for (idx, prefix) in PrefixIterator::new(&string, 4).enumerate() {
        let charset: HashSet<char> = HashSet::from_iter(prefix.chars());
        if charset.len() == 4 {
            return (idx + 4) as u32;
        };
    }
    panic!("No code found!");
}

fn solution2(mut lines: impl Iterator<Item = String>) -> u32 {
    let string = lines.next().expect("Expected single line");
    for (idx, prefix) in PrefixIterator::new(&string, 14).enumerate() {
        let charset: HashSet<char> = HashSet::from_iter(prefix.chars());
        if charset.len() == 14 {
            return (idx + 14) as u32;
        };
    }
    panic!("No code found!");
}

#[cfg(test)]
mod test {
    use crate::{solution1, solution2, PrefixIterator};
    use utils::string_lines;

    fn test_iter1() -> impl Iterator<Item = String> {
        string_lines(r"mjqjpqmgbljsphdztnvjfqwrcgsmlb")
    }

    fn test_iter2() -> impl Iterator<Item = String> {
        string_lines(r"bvwbjplbgvbhsrlpgdmjqwftvncz")
    }

    #[test]
    fn test_prefix_iter() {
        let iter = PrefixIterator::new("abc", 2);
        let prefixes: Vec<&str> = iter.collect();
        assert_eq!(prefixes, vec!["ab", "bc"]);
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(test_iter1()), 7);
        assert_eq!(solution1(test_iter2()), 5);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(test_iter1()), 19);
        assert_eq!(solution2(test_iter2()), 23);
    }
}

fn main() {
    println!("Solution 1: {}", solution1(lines("input.txt")));
    println!("Solution 2: {}", solution2(lines("input.txt")));
}
