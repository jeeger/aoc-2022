use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;

pub fn lines(filename: &str) -> impl Iterator<Item = String> {
    let r = BufReader::new(File::open(filename).expect("Failed to open file"));
    r.lines().map(|elem| elem.expect("Couldn't read line"))
}

pub fn split_lines(filename: &str) -> impl Iterator<Item = Vec<String>> {
    lines(filename).filter(|s| !s.is_empty()).map(|s| s.split_whitespace().map(|str| String::from(str)).collect())
}

pub fn string_lines(s: &str) -> impl Iterator<Item = String> + '_ {
    let r = BufReader::new(Cursor::new(s)).lines();
    r.map(|elem| elem.expect("Couldn't read line"))
}

pub fn string_split_lines(s: &str) -> impl Iterator<Item = Vec<String>> + '_ {
    println!("{}", s);
    string_lines(s).filter(|s| !s.is_empty()).map(|s| s.split_whitespace().map(|f| String::from(f)).collect())
}
