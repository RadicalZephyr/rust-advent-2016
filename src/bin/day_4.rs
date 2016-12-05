#[macro_use]
extern crate advent_2016;
extern crate nom;

use std::fs::File;
use std::io::Read;

use nom::IResult;

use advent_2016::day_4::Room;
use advent_2016::day_4::parse;

pub fn main() {
    let mut f = File::open("../../data/day_4.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Couldn't read file into string");
    match_parse!{
        rooms = parse::rooms(s.as_bytes()) => {
            part_1(&rooms);
            part_2(&rooms)
        }
    }
}

fn part_1(rooms: &Vec<Room>) {
    let sum_of_ids = rooms.iter()
        .filter(|r| r.is_real())
        .fold(0, |sum, r| sum + r.sector_id as u64);
    println!("Total of real sector ids: {}", sum_of_ids);
}

fn part_2(_rooms: &Vec<Room>) {
    let x = 26;
    let a = x % 26;
    println!("{} % 26 = {}", x, a);
    println!("{}", 'z' as u8 - 'a' as u8);
}
