#[macro_use]
extern crate advent_2016;
extern crate nom;

use std::fs::File;
use std::io::Read;

use nom::IResult;

use advent_2016::day_1::Instruction;
use advent_2016::day_1::parse;
use advent_2016::day_1::navigation;

pub fn main() {
    let mut f = File::open("../../data/day_1.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Couldn't read file into string");
    match_parse!{
        instructions = parse::instructions(s.as_bytes()) => {
            part_1(&instructions);
            part_2(&instructions);
        }
    }
}

fn part_1(instructions: &Vec<Instruction>) {
    let hq_location = navigation::Location::new().follow_all_instructions(instructions);
    println!("Final location is {:?}", hq_location)
}

fn part_2(instructions: &Vec<Instruction>) {
    let hq_location = navigation::Location::new().first_repeated_location(instructions);
    println!("Final location is {:?}", hq_location)
}
