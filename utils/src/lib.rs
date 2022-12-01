use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn lines(filename: &str) -> impl Iterator<Item = String> {
    let r = BufReader::new(File::open(filename).expect("Failed to open file"));
    r.lines().map(|elem| elem.expect("Couldn't read line"))
}
