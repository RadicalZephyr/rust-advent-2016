extern crate nom;
extern crate advent_2016;

use std::fs::File;
use std::io::Read;

use nom::IResult;

// use advent_2016::day_4;
use advent_2016::day_4::parse;

pub fn main() {
    let mut f = File::open("../../data/day_4.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Couldn't read file into string");
    match parse::things(s.as_bytes()) {
        IResult::Done(_, things) => {
            part_1(things.clone());
            part_2(things.clone())
        }
        IResult::Error(error) => panic!("Error: {:?}", error),
        IResult::Incomplete(needed) => panic!("Incomplete input: {:?}", needed),
    }
}

fn part_1(_list: Vec<()>) {}

fn part_2(_list: Vec<()>) {}
