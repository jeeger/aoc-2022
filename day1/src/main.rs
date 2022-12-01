use utils::lines;

fn solution_generic(filename: &str) -> Vec<u32> {
    let mut current_sum = 0;
    lines(filename).fold(Vec::new(), |mut l: Vec<u32>, s| {
        if s.is_empty() {
            l.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += s.parse::<u32>().expect("Failed to parse number");
        }
        l
    })
}

fn solution2(filename: &str) {
    let mut calories = solution_generic(filename);
    calories.sort();
    calories.reverse();
    calories.truncate(3);
    println!("Solution 2: {}", calories.iter().sum::<u32>())
}

fn solution1(filename: &str) {
    let mut calories = solution_generic(filename);
    calories.sort();
    calories.reverse();
    print!("Solution 1: {}\n", calories.first().expect("Empty calories list"));
}

fn main() {
    solution1("input1.txt");
    solution2("input2.txt");
}
