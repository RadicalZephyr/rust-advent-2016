#[macro_use]
extern crate advent_2016;
extern crate nom;

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
    match_parse!{
        instructions_list = parse::instructions_list(s.as_bytes()) => {
            part_1(&instructions_list);
            part_2(&instructions_list)
        }
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
