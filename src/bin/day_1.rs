#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::Read;
use std::str;
use std::str::FromStr;
use std::convert::From;

use nom::digit;

enum Direction {
    L,
    R,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            _ => Err(()),
        }
    }
}

struct Instruction {
    direction: Direction,
    distance: u8,
}

fn instruction_from(v: (Direction, u8)) -> Instruction {
    let (direction, distance) = v;

    Instruction {
        direction: direction,
        distance: distance,
    }
}

named!(direction<Direction>,
    map_res!(
        map_res!(
          alt!(tag!("L") | tag!("R")) ,
          str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(distance<u8>,
    map_res!(
      map_res!(
          digit,
          str::from_utf8
      ),
        FromStr::from_str
    )
);

named!(instruction<Instruction>,
    map!(
        pair!(direction, distance),
        instruction_from
    )
);

named!(instructions<Vec<Instruction> >,
       many1!(ws!(instruction))
);

pub fn main() {
    let mut f = File::open("../../data/day_1.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Couldn't read file into string");

}

#[cfg(test)]
pub mod test {}
