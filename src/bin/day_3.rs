#[macro_use]
extern crate advent_2016;
extern crate nom;

use std::fs::File;
use std::io::Read;

use nom::IResult;

use advent_2016::day_3::Triangle;
use advent_2016::day_3::parse;

pub fn main() {
    let mut f = File::open("../../data/day_3.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Couldn't read file into string");
    match_parse!{
        triangles = parse::by_row_triangles(s.as_bytes()) => part_1(triangles)
    }
    match_parse!{
        triangles = parse::by_col_triangles(s.as_bytes()) => part_2(triangles)
    }
}

fn part_1(triangles: Vec<Triangle>) {
    let possible_triangle_count = triangles.iter().filter(|t| t.valid()).count();
    println!("Total column-parsed possible triangles: {}",
             possible_triangle_count);
}

fn part_2(triangles: Vec<Triangle>) {
    let possible_triangle_count = triangles.iter().filter(|t| t.valid()).count();
    println!("Total row-parsed possible triangles: {}",
             possible_triangle_count);
}
