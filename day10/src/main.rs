use std::{convert::Infallible, str::FromStr};
use utils::{lines, num_between, Point};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(isize),
}

trait ExecuteInstruction {
    fn cycles(&self) -> u32;
    fn execute(&self, counter: &mut isize);
}

impl ExecuteInstruction for Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }

    fn execute(&self, counter: &mut isize) {
        match self {
            Instruction::Noop => {}
            Instruction::Addx(c) => {
                *counter += c;
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Instruction::Noop)
        } else {
            Ok(Instruction::Addx(num_between(s, Some(" "), None) as isize))
        }
    }
}

fn parse_instructions(lines: impl Iterator<Item = String>) -> Vec<Instruction> {
    lines
        .map(|l| l.parse().expect("Failed to parse step"))
        .collect()
}

fn solution1(lines: impl Iterator<Item = String>) -> isize {
    let mut cycle = 0;
    let mut register = 1;
    let mut result = 0;
    for instruction in parse_instructions(lines) {
        for _i in 0..instruction.cycles() {
            cycle += 1;
            if (cycle == 20) || ((cycle - 20) % 40 == 0) {
                result += cycle * register;
            }
        }
        instruction.execute(&mut register);
    }
    result
}

fn cycle_to_coord(c: u32) -> Point {
    if c == 1 {
        return Point::new(0, 0);
    };
    let row = (c - 1) / 40;
    let col = (c - 1) % 40;
    Point::new(col as isize, row as isize)
}
 
fn solution2(lines: impl Iterator<Item = String>) -> String {
    let mut cycle = 1;
    let mut register: isize = 1;
    let mut result: Vec<Vec<char>> = std::iter::repeat_with(|| {
        std::iter::repeat('.').take(40).collect()
    }).take(6).collect();
    for instruction in parse_instructions(lines) {
        for _i in 0..instruction.cycles() {
            let pos = cycle_to_coord(cycle);
            if (register-1..register+2).contains(&pos.x) {
                result[pos.y as usize][pos.x as usize] = '#';
            }
            cycle += 1;
        }
        instruction.execute(&mut register);
    }
    result.iter().map(|r| {
        String::from_iter(r)
    }).collect::<Vec<String>>().join("\n")
}

#[cfg(test)]
mod test {
    use crate::{solution1, solution2, cycle_to_coord};
    use utils::{string_lines, Point};

    fn test_iter() -> impl Iterator<Item = String> {
        string_lines(
            r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        )
    }

    #[test]
    fn test_position() {
        assert_eq!(cycle_to_coord(1), Point::new(0, 0));
        assert_eq!(cycle_to_coord(40), Point::new(39, 0));
        assert_eq!(cycle_to_coord(1), Point::new(0, 0));
        assert_eq!(cycle_to_coord(40), Point::new(39, 0));
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(test_iter()), 13140);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(test_iter()), r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
    }
}

fn main() {
    println!("Solution 1: {}", solution1(lines("input.txt")));
    println!("Solution 2: \n{}", solution2(lines("input.txt")));
}
