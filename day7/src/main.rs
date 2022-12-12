#![feature(iter_array_chunks)]
#![feature(map_many_mut)]
use std::collections::HashMap;
use utils::{lines, num_between, str_between};

fn increase_size(sizes: &mut HashMap<String, u32>, path: String, to_increase: u32) {
    let new_size = *sizes.get(&path).unwrap_or(&0) + to_increase;
    sizes.insert(path.to_string(), new_size);
}

fn parse_directory(lines: &mut impl Iterator<Item = String>) -> HashMap<String, u32> {
    let mut result: HashMap<String, u32> = HashMap::new();
    let mut current_path: Vec<String> = Vec::new();
    for line in lines {
        let current_path_str = current_path.join("/");
        match line {
            line if line == "$ cd /" => {
                // This could be ignored, only happens once on first line.
                let subdir_size = *result.get(&current_path_str).unwrap_or(&0);
                while let Some(_p) = current_path.pop() {
                    increase_size(&mut result, current_path.join("/"), subdir_size);
                }
                increase_size(&mut result, "".to_string(), subdir_size);
            }
            line if line == "$ cd .." => {
                let subdir_size = result[&current_path_str];
                current_path.pop();
                let parent_str = current_path.join("/");
                increase_size(&mut result, parent_str.to_owned(), subdir_size);
            }
            // CD command, not root.
            line if line.starts_with("$ cd ") => {
                let dirname = str_between(&line, Some("$ cd "), None);
                current_path.push(dirname.to_owned());
                increase_size(&mut result, current_path.join("/"), 0);
            }
            // Directory in ls output
            line if line.starts_with("dir ") => {}
            line if line.starts_with("$ ls") => {}
            line => {
                let size = num_between(&line, None, Some(" "));
                increase_size(&mut result, current_path_str, size.try_into().unwrap());
            }
        }
    }
    let mut last_subdir_size = *result.get(&current_path.join("/")).unwrap_or(&0);
    while let Some(_p) = current_path.pop() {
        let path = current_path.join("/");
        increase_size(&mut result, path.to_owned(), last_subdir_size);
        last_subdir_size = result[&path];
    }
    return result;
}

fn solution1(mut lines: impl Iterator<Item = String>) -> u32 {
    parse_directory(&mut lines)
        .values()
        .filter(|&v| *v <= 100000)
        .cloned()
        .sum()
}

fn solution2(mut lines: impl Iterator<Item = String>) -> u32 {
    let directory_sizes = parse_directory(&mut lines);
    let total_size = directory_sizes[""];
    let need_to_free: i64 = (total_size as i64) - 40000000;
    if need_to_free <= 0 {
        return 0;
    }
    let mut valid_deletions: Vec<u32> = directory_sizes
        .values()
        .filter(|&v| *v >= (need_to_free as u32))
        .cloned()
        .collect();
    valid_deletions.sort();
    *valid_deletions
        .first()
        .expect("No valid directory to delete")
}

#[cfg(test)]
mod test {
    use crate::{parse_directory, solution1, solution2};
    use std::collections::HashMap;
    use utils::string_lines;

    fn test_iter1() -> impl Iterator<Item = String> {
        string_lines(
            r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
",
        )
    }

    /**
    /a: 10
    /a/d: 4
    /b: 18
    /c: 0
    **/
    fn test_iter2() -> impl Iterator<Item = String> {
        string_lines(
            r"$ cd /
$ ls
dir a
dir b
$ cd a
$ ls
dir a
1 e
$ cd a
$ ls
2 i
",
        )
    }

    #[test]
    fn test_sizes() {
        assert_eq!(
            parse_directory(&mut test_iter1()),
            HashMap::from([
                (String::from("a/e"), 584),
                (String::from("a"), 94853),
                (String::from("d"), 24933642),
                (String::from(""), 48381165)
            ])
        );
    }

    #[test]
    fn test_sizes2() {
        assert_eq!(
            parse_directory(&mut test_iter2()),
            HashMap::from([
                (String::from("a"), 3),
                (String::from("a/a"), 2),
                (String::from(""), 3)
            ])
        );
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(test_iter1()), 95437);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(test_iter1()), 24933642);
    }
}

fn main() {
    println!("Solution 1: {}", solution1(lines("input.txt")));
    println!("Solution 2: {}", solution2(lines("input.txt")));
}
