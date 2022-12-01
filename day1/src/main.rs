use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::BinaryHeap;

fn solution2(filename: &str) -> Result<(), std::io::Error> {
    let r = BufReader::new(File::open(filename)?);
    let mut h = BinaryHeap::new();
    let mut current_sum: u32 = 0;
    for line in r.lines() {
        match line {
            Ok(s) if s.is_empty() => {
                h.push(current_sum);
                current_sum = 0;
            }
            Ok(s) => {
                current_sum += s.parse::<u32>().expect("Failed to parse number");
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    let mut solution = 0;
    for _n in 1..4 {
        solution += h.pop().expect("Empty heap");
    }
    print!("Solution 2: {}\n", solution);
    Ok(())
}

fn solution1(filename: &str) -> Result<(), std::io::Error> {
    let mut current_max = 0;
    let mut current_sum = 0;
    let path = Path::new(filename);
    let f = File::open(path)?;
    let buf = BufReader::new(f);
    for line in buf.lines() {
        match line {
            Ok(s) if s.is_empty() => {
                if current_sum > current_max {
                    current_max = current_sum;
                }
                current_sum = 0;
            }
            Ok(s) => {
                let num: u32 = s.parse().expect("Failed to parse number");
                current_sum += num;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    print!("Solution 1: {}\n", current_max);
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    solution1("input1.txt")?;
    solution2("input2.txt")
}
