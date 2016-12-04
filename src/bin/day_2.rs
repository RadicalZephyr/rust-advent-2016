extern crate nom;
extern crate advent_2016;

use std::fs::File;
use std::io::Read;

use nom::IResult;

use advent_2016::day_2;
use advent_2016::day_2::{Direction, Keypad};
use advent_2016::day_2::parse;

pub fn main() {
    let mut f = File::open("../../data/day_2.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Couldn't read file into string");
    match parse::instructions_list(s.as_bytes()) {
        IResult::Done(_, instructions_list) => {
            part_1(&instructions_list);
            part_2(&instructions_list)
        }
        IResult::Error(error) => panic!("Error: {:?}", error),
        IResult::Incomplete(needed) => panic!("Incomplete input: {:?}", needed),
    }
}

fn part_1(instructions_list: &Vec<Vec<Direction>>) {
    let mut kp = Keypad::new(day_2::simple_lookup());
    for instructions in instructions_list {
        kp.next_key(instructions);
        print!("{} ", kp.num);
    }
    println!("");
}

fn part_2(instructions_list: &Vec<Vec<Direction>>) {
    let mut kp = Keypad::new(day_2::crazy_lookup());
    for instructions in instructions_list {
        kp.next_key(instructions);
        print!("{} ", kp.num);
    }
    println!("");
}
