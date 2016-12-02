extern crate advent_2016;

use std::fs::File;
use std::io::Read;

use advent_2016::day_1::parse;

pub fn main() {
    let mut f = File::open("../../data/day_1.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Couldn't read file into string");
    let ists = parse::instructions(s.as_bytes());
}
