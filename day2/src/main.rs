use std::{convert::Infallible, str::FromStr};
use std::cmp::Ordering;
use utils::split_lines;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RPS {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => panic!("Failed to parse RPS state {}", s),
        }
    }
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match (self, other) {
            (RPS::Paper, RPS::Rock) |
            (RPS::Scissors, RPS::Paper) |
            (RPS::Rock, RPS::Scissors) => Ordering::Greater,
            (RPS::Rock, RPS::Paper) |
            (RPS::Paper, RPS::Scissors) |
            (RPS::Scissors, RPS::Rock) => Ordering::Less,
            (RPS::Rock, RPS::Rock) |
            (RPS::Paper, RPS::Paper) |
            (RPS::Scissors, RPS::Scissors) => Ordering::Equal
        })
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum DesiredOutcome {
    Win,
    Draw,
    Lose,
}

impl FromStr for DesiredOutcome {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(DesiredOutcome::Lose),
            "Y" => Ok(DesiredOutcome::Draw),
            "Z" => Ok(DesiredOutcome::Win),
            _ => panic!("Failed to parse desired game state {}", s),
        }
    }
}

fn wins_against(r: RPS) -> RPS {
    match r {
        RPS::Rock => RPS::Paper,
        RPS::Paper => RPS::Scissors,
        RPS::Scissors => RPS::Rock,
    }
}

fn loses_against(r: RPS) -> RPS {
    match r {
        RPS::Paper => RPS::Rock,
        RPS::Rock => RPS::Scissors,
        RPS::Scissors => RPS::Paper
    }
}

fn shape_score(r: RPS) -> u32 {
    match r {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn score_rps(theirs: RPS, mine: RPS) -> u32 {
    (match (theirs, mine) {
        (x, y) if x == y => 3,
        (x, y) if x > y => 0,
        (x, y) if x < y => 6,
        (x, y) => panic!("Unkown RPS battle {:?} vs {:?}", x, y),
    }) + shape_score(mine)
}

fn solution1(lines: impl Iterator<Item = Vec<String>>) -> u32 {
    let mut result_score = 0;
    for game in lines {
        let theirs: RPS = game[0].parse().expect("Failed to parse RPS value");
        let ours: RPS = game[1].parse().expect("Failed to parse RPS value");
        result_score += score_rps(theirs, ours);
    }
    result_score
}

fn shape_for_outcome(theirs: RPS, outcome: DesiredOutcome) -> RPS {
    match (theirs, outcome) {
        (x, DesiredOutcome::Draw) => x,
        (x, DesiredOutcome::Win) => wins_against(x),
        (x, DesiredOutcome::Lose) => loses_against(x)
    }
}

fn solution2(lines: impl Iterator<Item = Vec<String>>) -> u32 {
    let mut result_score = 0;
    for game in lines {
        let theirs: RPS = game[0].parse().expect("Failed to parse RPS value");
        let outcome: DesiredOutcome = game[1].parse().expect("Failed to parse desired outcome");
        let ours = shape_for_outcome(theirs, outcome);
        let score = score_rps(theirs, ours);
        result_score += score;
    }
    result_score
}

#[cfg(test)]
mod test {
    use crate::{solution1, solution2};
    use utils::string_split_lines;

    fn test_iter() -> impl Iterator<Item = Vec<String>> {
        string_split_lines(
            r"
A Y
B X
C Z
",
        )
    }

    
    #[test]
    fn test_solution1() {
        assert_eq!(solution1(test_iter()), 15);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(test_iter()), 12);
    }
}

fn main() {
    println!("Solution 1: {}", solution1(split_lines("input.txt")));
    println!("Solution 2: {}", solution2(split_lines("input.txt")));
}
