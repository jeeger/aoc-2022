#![feature(iter_array_chunks)]
#![feature(map_many_mut)]
use std::collections::HashMap;
use utils::{lines, num_between};

type BoxConfig = HashMap<u32, Vec<char>>;

fn apply_order(cfg: &mut BoxConfig, o: Order, do_rev: bool) {
    let [from_vec, to_vec] = cfg.get_many_mut([&o.from_col, &o.to_col]).expect("Invalid order");
    let drain = from_vec.drain(0..(o.count as usize));
    if do_rev {
        drain.rev().for_each(|b| to_vec.insert(0, b))
    } else {
        drain.for_each(|b| to_vec.insert(0, b))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Order {
    count: u32,
    from_col: u32,
    to_col: u32,
}

impl Order {
    fn new(count: u32, from_col: u32, to_col: u32) -> Self {
        Order {
            count,
            from_col,
            to_col,
        }
    }
}

fn box_config(lines: &mut impl Iterator<Item = String>) -> Vec<String> {
    lines.take_while(|line| !line.is_empty()).collect()
}

fn get_solution_code(b: &BoxConfig) -> String {
    let mut columns: Vec<u32> = b.keys().cloned().collect();
    let mut result = String::new();
    columns.sort();
    for column in columns {
        result.push(b[&column].first().unwrap().clone());
    }
    result
}

fn parse_box_config(lines: Vec<String>) -> BoxConfig {
    let mut result: BoxConfig = HashMap::new();
    for line in lines {
        for (idx, char) in line.chars().enumerate() {
            if char.is_uppercase() && char.is_alphabetic() {
                let col: u32 = ((idx as u32 - 1) / 4) + 1;
                match result.get_mut(&col) {
                    Some(current_col) => current_col.push(char),
                    None => {
                        result.insert(col, vec![char]);
                        ()
                    }
                };
            }
        }
    }
    result
}
fn parse_orders(lines: Vec<String>) -> Vec<Order> {
    let mut result = Vec::new();
    for line in lines {
        let count: u32 = num_between(&line, Some("move "), Some(" from"));
        let from_col: u32 = num_between(&line, Some("from "), Some(" to"));
        let to_col: u32 = num_between(&line, Some("to "), None);
        result.push(Order::new(count, from_col, to_col));
    }
    return result;
}

fn solution1(mut lines: impl Iterator<Item = String>) -> String {
    let mut box_config = parse_box_config(box_config(&mut lines));
    let instructions = parse_orders(lines.collect::<Vec<String>>());
    for instruction in instructions {
        apply_order(&mut box_config, instruction, false);
    }
    get_solution_code(&box_config)
}

fn solution2(mut lines: impl Iterator<Item = String>) -> String {
    let mut box_config = parse_box_config(box_config(&mut lines));
    let instructions = parse_orders(lines.collect::<Vec<String>>());
    for instruction in instructions {
        apply_order(&mut box_config, instruction, true);
    }
    get_solution_code(&box_config)
}

#[cfg(test)]
mod test {
    use crate::{box_config, parse_box_config, solution1, solution2, parse_orders, Order, apply_order, get_solution_code};
    use std::collections::HashMap;
    use utils::string_lines;

    fn test_iter() -> impl Iterator<Item = String> {
        string_lines(

r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        )
    }

    #[test]
    fn test_parsing_columns() {
        let box_config = box_config(&mut test_iter());
        let parsed_config = parse_box_config(box_config);
        let expected = HashMap::from([
            (1, vec!['N', 'Z']),
            (2, vec!['D', 'C', 'M']),
            (3, vec!['P']),
        ]);
        assert_eq!(parsed_config, expected);
    }

    #[test]
    fn test_parsing_orders() {
        let mut input = test_iter();
        box_config(&mut input); 
        let parsed_orders = parse_orders(input.collect());
        let expected = vec![
            Order::new(1, 2, 1),
            Order::new(3, 1, 3),
            Order::new(2, 2, 1),
            Order::new(1, 1, 2)
        ];
        assert_eq!(parsed_orders, expected);
    }

    #[test]
    fn test_apply_order() {
        let mut state = HashMap::from([
            (1, vec!['N', 'Z']),
            (2, vec!['D', 'C', 'M']),
            (3, vec!['P']),
        ]);
        let order = Order::new(1, 1, 2);
        let expected = HashMap::from([
            (1, vec!['Z']),
            (2, vec!['N', 'D', 'C', 'M']),
            (3, vec!['P']),
        ]);
        apply_order(&mut state, order, false);
        assert_eq!(state, expected);
    }

    #[test]
    fn test_solution_code() {
        let state = HashMap::from([
            (1, vec!['N', 'Z']),
            (2, vec!['D', 'C', 'M']),
            (3, vec!['P']),
        ]);
        assert_eq!("NDP", get_solution_code(&state));
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(test_iter()), "CMZ");
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(test_iter()), "MCD");
    }
}

fn main() {
    println!("Solution 1: {}", solution1(lines("input.txt")));
    println!("Solution 2: {}", solution2(lines("input.txt")));
}
