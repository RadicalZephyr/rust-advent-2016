extern crate nom;
extern crate advent_2016;

use std::fs::File;
use std::io::Read;

use nom::IResult;

use advent_2016::day_1::parse;
use advent_2016::day_1::navigation;

pub fn main() {
    let mut f = File::open("../../data/day_1.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Couldn't read file into string");
    match parse::instructions(s.as_bytes()) {
        IResult::Done(_, instructions) => {
            // let hq_location = navigation::Location::new().follow_all_instructions(instructions);
            let hq_location = navigation::Location::new().first_repeated_location(instructions);
            println!("Final location is {:?}", hq_location)
        }
        IResult::Error(error) => panic!("Error: {:?}", error),
        IResult::Incomplete(needed) => panic!("Incomplete input: {:?}", needed),
    }
}
