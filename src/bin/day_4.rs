extern crate nom;
extern crate advent_2016;

use std::fs::File;
use std::io::Read;

use nom::IResult;

use advent_2016::day_4::Room;
use advent_2016::day_4::parse;

pub fn main() {
    let mut f = File::open("../../data/day_4.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Couldn't read file into string");
    match parse::rooms(s.as_bytes()) {
        IResult::Done(_, rooms) => {
            part_1(rooms.clone());
            part_2(rooms.clone())
        }
        IResult::Error(error) => panic!("Error: {:?}", error),
        IResult::Incomplete(needed) => panic!("Incomplete input: {:?}", needed),
    }
}

fn part_1(_rooms: Vec<Room>) {}

fn part_2(_rooms: Vec<Room>) {}
