use std::collections::HashMap;

use utils::{lines, num_between, numbers_on_line};

struct Monkey {
    starting_items: Vec<u32>,
    operation: Box<dyn Fn(&u32) -> u32>,
    test: Box<dyn Fn(&u32) -> bool>,
    true_monkey: u32,
    false_monkey: u32,
}

fn parse_operation(s: &str) -> Box<dyn Fn(&u32) -> u32> {
    let split: Vec<String> = s.split_whitespace().map(|s| String::from(s)).collect();
    if split[2] == "old" {
        Box::new(|x: &u32| -> u32 { x * x })
    } else if split[1] == "+" {
        let toadd = split[2].parse::<u32>().unwrap();
        Box::new(move |x: &u32| -> u32 { x + toadd })
    } else if split[1] == "*" {
        let toadd = split[2].parse::<u32>().unwrap();
        Box::new(move |x: &u32| -> u32 { x * toadd })
    } else {
        panic!("Unable to parse operation {}", s);
    }
}

fn parse_test(s: &str) -> Box<dyn Fn(&u32) -> bool> {
    let divisor: u32 = num_between(s, Some("divisible by "), None)
        .try_into()
        .unwrap();
    Box::new(move |x: &u32| -> bool { x % divisor == 0 })
}

fn parse_monkey(lines: &mut impl Iterator<Item = String>) -> Monkey {
    let starting_items: Vec<u32> = numbers_on_line(&lines.next().unwrap())
        .iter()
        .copied()
        .map(|i| i.try_into().unwrap())
        .collect();
    let operation = parse_operation(&lines.next().unwrap()[19..]);
    let test = parse_test(&lines.next().unwrap()[8..]);
    let true_monkey = num_between(&lines.next().unwrap(), Some("to monkey "), None);
    let false_monkey = num_between(&lines.next().unwrap(), Some("to monkey "), None);
    Monkey {
        starting_items,
        operation,
        test,
        true_monkey: true_monkey.try_into().unwrap(),
        false_monkey: false_monkey.try_into().unwrap(),
    }
}

fn parse_all_monkeys(lines: &mut impl Iterator<Item = String>) -> HashMap<u32, Monkey> {
    let mut monkeys = HashMap::new();
    let mut i = 0;
    while let Some(_line) = lines.next() {
        monkeys.insert(i, parse_monkey(lines));
        // Drop trailing newline
        lines.next();
        i += 1;
    }
    monkeys
}

fn monkey_business(inspections: &HashMap<u32, u32>) -> u32 {
    let mut inspection_count: Vec<u32> = inspections.values().copied().collect();
    inspection_count.sort();
    inspection_count.reverse();
    let most_active = inspection_count[0];
    let second_most_active = inspection_count[1];
    most_active * second_most_active
}

fn solution1(mut lines: impl Iterator<Item = String>) -> u32 {
    let mut monkeys = parse_all_monkeys(&mut lines);
    let mut inspections: HashMap<u32, u32> = HashMap::new();
    for round in 0..20 {
        for i in 0..monkeys.len() as u32 {
            let current_monkey = monkeys.get_mut(&i).unwrap();
            let item_count: u32 = current_monkey.starting_items.len().try_into().unwrap();
            inspections
                .entry(i.try_into().unwrap())
                .and_modify(|e| *e += item_count)
                .or_insert(item_count);
            let mut to_append: HashMap<u32, Vec<u32>> = HashMap::new();
            for item in &current_monkey.starting_items {
                let mut new_item = (current_monkey.operation)(&item);
                new_item = new_item / 3;
                if (current_monkey.test)(&new_item) {
                    let new_monkey_index: u32 = current_monkey.true_monkey;
                    to_append.entry(new_monkey_index).and_modify(|v| v.push(new_item)).or_insert(vec![new_item]);
                } else {
                    let new_monkey_index: u32 = current_monkey.false_monkey;
                    to_append.entry(new_monkey_index).and_modify(|v| v.push(new_item)).or_insert(vec![new_item]);
                }
            }
            current_monkey.starting_items.clear();
            for (k, v) in to_append {
                monkeys.get_mut(&k).unwrap().starting_items.extend(v);
            }
        }
    }
    return monkey_business(&inspections);
}

fn solution2(_lines: impl Iterator<Item = String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use crate::{parse_all_monkeys, parse_monkey, solution1, solution2};
    use utils::string_lines;

    fn test_iter() -> impl Iterator<Item = String> {
        string_lines(
            r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        )
    }

    #[test]
    fn test_parse_monkey() {
        let mut iter = test_iter();
        iter.next();
        let monkey = parse_monkey(&mut iter);
        assert_eq!(monkey.starting_items, Vec::from_iter([79, 98]));
        assert_eq!((monkey.operation)(&15), 15 * 19);
        assert_eq!((monkey.test)(&2), false);
        assert_eq!((monkey.test)(&23), true);
        assert_eq!(monkey.true_monkey, 2);
        assert_eq!(monkey.false_monkey, 3);
    }

    #[test]
    fn test_parse_all_monkeys() {
        let monkeys = parse_all_monkeys(&mut test_iter());
        assert_eq!(monkeys.len(), 4);
        assert_eq!(monkeys[&0].starting_items, vec![79, 98]);
        assert_eq!(monkeys[&1].starting_items, vec![54, 65, 75, 74]);
        assert_eq!(monkeys[&3].starting_items, vec![74]);
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(test_iter()), 10605);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(test_iter()), "");
    }
}

fn main() {
    println!("Solution 1: {}", solution1(lines("input.txt")));
    //println!("Solution 2: \n{}", solution2(lines("input.txt")));
}
