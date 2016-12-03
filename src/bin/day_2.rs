extern crate nom;
extern crate advent_2016;

use std::fs::File;
use std::io::Read;

pub fn main() {
    let mut f = File::open("../../data/day_2.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Couldn't read file into string");

}
