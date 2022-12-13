#![allow(dead_code, unused_variables)]
extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{newline, u32},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};
use std::{cmp::Ordering, fs::read_to_string};

#[derive(Eq, PartialEq, Clone)]
enum Data {
    Value(u32),
    Data(Vec<Box<Data>>),
}

impl Data {
    fn values(v: &[u32]) -> Self {
        Data::Data(v.iter().map(|v| Box::new(Data::Value(*v))).collect())
    }
    fn data(d: Vec<Data>) -> Self {
        Data::Data(d.into_iter().map(|d| Box::new(d)).collect())
    }
}

fn box_vec(v: Vec<Data>) -> Data {
    Data::Data(v.into_iter().map(|d| Box::new(d)).collect())
}

fn parse_line(line: &str) -> IResult<&str, Data> {
    let parse_value = map(u32, |u| Data::Value(u));
    let mut parse_list = alt((
        parse_value,
        delimited(
            char('['),
            map(separated_list0(tag(","), parse_line), box_vec),
            char(']'),
        ),
    ));
    parse_list(line)
}

fn parse_data_pair(text: &str) -> IResult<&str, (Data, Data)> {
    let (rest, (first, _, second, _)) = tuple((parse_line, newline, parse_line, newline))(text)?;
    Ok((rest, (first, second)))
}

fn parse_input(text: &str) -> IResult<&str, Vec<(Data, Data)>> {
    separated_list1(newline, parse_data_pair)(text)
}

fn compare(left: &Data, right: &Data) -> Ordering {
    match (left, right) {
        (Data::Value(l), Data::Value(r)) => l.cmp(&r),
        (Data::Value(l), Data::Data(r)) => {
            compare(&Data::Data(Vec::from([Box::new(Data::Value(*l))])), right)
        }
        (Data::Data(l), Data::Value(r)) => {
            compare(left, &Data::Data(Vec::from([Box::new(Data::Value(*r))])))
        }
        (Data::Data(l), Data::Data(r)) if l.len() == 0 && r.len() == 0 => Ordering::Equal,
        (Data::Data(l), Data::Data(r)) if l.len() == 0 => Ordering::Less,
        (Data::Data(l), Data::Data(r)) if r.len() == 0 => Ordering::Greater,
        (Data::Data(l), Data::Data(r)) => {
            for (l_elem, r_elem) in l.iter().zip(r.iter()) {
                let order = compare(l_elem, r_elem);
                if order != Ordering::Equal {
                    return order;
                }
            }
            if l.len() < r.len() {
                return Ordering::Less;
            } else if r.len() < l.len() {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        }
    }
}

fn solution_1_calc(d: Vec<(Data, Data)>) -> u32 {
    let mut result: u32 = 0;
    for (idx, (l, r)) in d.iter().enumerate() {
        if compare(l, r) == Ordering::Less {
            result = result + (idx as u32) + 1;
        }
    }
    result
}

fn solution1(input: &str) -> u32 {
    match parse_input(input) {
        Ok((rest, pairs)) => solution_1_calc(pairs),
        Err(e) => panic!("{}", e),
    }
}

fn flatten_pairs(v: Vec<(Data, Data)>) -> Vec<Data> {
    v.into_iter().flat_map(|(d1, d2)| [d1, d2]).collect()
}

fn solution2(input: &str) -> u32 {
    let mut data = match parse_input(input) {
        Ok((rest, pairs)) => flatten_pairs(pairs),
        Err(e) => panic!("{}", e),
    };
    let marker_first = Data::data(vec![Data::values(&[2])]);
    let marker_second = Data::data(vec![Data::values(&[6])]);
    data.push(marker_first.clone());
    data.push(marker_second.clone());
    data.sort_by(compare);
    let first_pos = data.iter().position(|d| *d == marker_first).unwrap() + 1;
    let second_pos = data.iter().position(|d| *d == marker_second).unwrap() + 1;
    (first_pos as u32) * (second_pos as u32)
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use crate::{compare, parse_data_pair, parse_input, parse_line, solution1, solution2};

    const TEST_STRING: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test_parse_line() {
        let test_string = r"[1,2,3]";
        match parse_line(test_string) {
            Ok((rest, output)) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    fn compare_pair(input: &str) -> Ordering {
        let (left, right) = parse_data_pair(input).unwrap().1;
        compare(&left, &right)
    }

    #[test]
    fn test_parser_pair() {
        let test_string = r"[1,1,3,1,1]
[1,1,5,1,1]
";
        match parse_data_pair(test_string) {
            Ok((rest, output)) => {
                assert!(rest.is_empty());
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_parse_solution() {
        match parse_input(TEST_STRING) {
            Ok((rest, output)) => {
                assert!(rest.is_empty());
                assert_eq!(output.len(), 8);
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_compare_simple() {
        assert_eq!(compare_pair("[1,1,3,1,1]\n[1,1,5,1,1]\n"), Ordering::Less);
        assert_eq!(compare_pair("[[1],[2,3,4]]\n[[1],4]\n"), Ordering::Less);
        assert_eq!(compare_pair("[9]\n[[8,7,6]]\n"), Ordering::Greater);
        assert_eq!(compare_pair("[[4,4],4,4]\n[[4,4],4,4,4]\n"), Ordering::Less);
        assert_eq!(compare_pair("[7,7,7,7]\n[7,7,7]\n"), Ordering::Greater);
        assert_eq!(compare_pair("[]\n[3]\n"), Ordering::Less);
        assert_eq!(compare_pair("[[[]]]\n[[]]\n"), Ordering::Greater);
        assert_eq!(
            compare_pair("[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]\n"),
            Ordering::Greater
        );
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(TEST_STRING), 13);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(TEST_STRING), 140);
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
