use utils::lines;

#[cfg(test)]
mod test {
    use utils::string_lines;
    use crate::solution1;
    use crate::solution2;

    #[test]
    fn test_solution2() {
        let test_iter = string_lines(
            r"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
",
        );
        assert_eq!(solution2(test_iter), 45000)
    }

    #[test]
    fn test_solution1() {
        let test_iter = string_lines(
            r"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
",
        );
        assert_eq!(solution1(test_iter), 24000);
    }
}

fn solution_generic(iter: impl Iterator<Item = String>) -> Vec<u32> {
    let mut current_sum = 0;
    let mut result = iter.fold(Vec::new(), |mut l: Vec<u32>, s| {
        if s.is_empty() {
            l.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += s.parse::<u32>().expect("Failed to parse number");
        }
        l
    });
    // Last entry is not necessarily followed by an empty line.
    result.push(current_sum);
    result
}

fn solution2(iter: impl Iterator<Item = String>) -> u32 {
    let mut calories = solution_generic(iter);
    calories.sort_unstable();
    calories.reverse();
    calories.truncate(3);
    calories.iter().sum::<u32>()
}

fn solution1(iter: impl Iterator<Item = String>) -> u32 {
    let mut calories = solution_generic(iter);
    calories.sort_unstable();
    calories.reverse();
    *calories.first().expect("Empty calories list")
}

fn main() {
    println!("Solution 1: {}", solution1(lines("input1.txt")));
    println!("Solution 2: {}", solution2(lines("input2.txt")));
}
