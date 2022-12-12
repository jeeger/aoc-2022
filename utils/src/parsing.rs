use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;

pub fn lines(filename: &str) -> impl Iterator<Item = String> {
    let r = BufReader::new(File::open(filename).expect("Failed to open file"));
    r.lines().map(|elem| elem.expect("Couldn't read line"))
}

pub fn split_lines(filename: &str) -> impl Iterator<Item = Vec<String>> {
    lines(filename)
        .filter(|s| !s.is_empty())
        .map(|s| s.split_whitespace().map(|str| String::from(str)).collect())
}

pub fn split_lines_sep(filename: &str, sep: char) -> impl Iterator<Item = Vec<String>> {
    lines(filename)
        .filter(|s| !s.is_empty())
        .map(move |s| s.split(sep).map(|str| String::from(str)).collect())
}

pub fn char_lines(filename: &str) -> impl Iterator<Item = Vec<char>> {
    lines(filename)
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
}

pub fn string_lines(s: &str) -> impl Iterator<Item = String> + '_ {
    let r = BufReader::new(Cursor::new(s)).lines();
    r.map(|elem| elem.expect("Couldn't read line"))
}

pub fn string_split_lines(s: &str) -> impl Iterator<Item = Vec<String>> + '_ {
    println!("{}", s);
    string_lines(s)
        .filter(|s| !s.is_empty())
        .map(|s| s.split_whitespace().map(|f| String::from(f)).collect())
}

pub fn string_char_lines(s: &str) -> impl Iterator<Item = Vec<char>> + '_ {
    string_lines(s)
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
}

pub fn string_split_lines_sep(s: &str, sep: char) -> impl Iterator<Item = Vec<String>> + '_ {
    string_lines(s)
        .filter(|s| !s.is_empty())
        .map(move |s| s.split(sep).map(|str| String::from(str)).collect())
}

pub fn str_between<'a>(line: &'a str, left: Option<&str>, right: Option<&str>) -> &'a str {
    let left_offset = left
        .map(|s| line.find(s).expect("Could not find left separator") + s.len())
        .unwrap_or(0);
    let right_offset = right
        .map(|s| line.find(s).expect("Could not find right separator"))
        .unwrap_or(line.len());
    &line[left_offset..right_offset]
}

pub fn num_between(line: &str, left: Option<&str>, right: Option<&str>) -> i32 {
    str_between(line, left, right).parse::<i32>().expect(
        format!(
            "Could not parse num between '{:?}' and '{:?}' in line {}",
            left, right, line
        )
        .as_str(),
    )
}
