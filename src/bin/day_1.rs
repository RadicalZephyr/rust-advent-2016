#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::Read;
use std::str;
use std::str::FromStr;

use nom::digit;

#[derive(Debug,PartialEq)]
pub enum Direction {
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

#[derive(Debug,PartialEq)]
pub struct Instruction {
    direction: Direction,
    distance: u8,
}

pub fn instruction_from(v: (Direction, u8)) -> Instruction {
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

named!(pub instruction<Instruction>,
    map!(
        pair!(direction, distance),
        instruction_from
    )
);

named!(pub instructions<Vec<Instruction> >,
       many1!(ws!(instruction))
);

pub fn main() {
    let mut f = File::open("../../data/day_1.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Couldn't read file into string");

}

#[cfg(test)]
pub mod test {
    use super::*;

    use nom::{ErrorKind, IResult};

    #[test]
    pub fn test_parse_instruction() {
        let i = instruction_from((Direction::L, 1));
        assert_eq!(instruction(b"L1"), IResult::Done(&b""[..], i))
    }

    #[test]
    pub fn test_fail_parse_instruction() {
        assert_eq!(instruction(b"D1"), IResult::Error(ErrorKind::Alt))
    }

    #[test]
    pub fn test_parse_many_instructions() {
        let l1 = instruction_from((Direction::L, 1));
        let r4 = instruction_from((Direction::R, 4));
        let is = vec![l1, r4];
        assert_eq!(instructions(b"L1 R4"), IResult::Done(&b""[..], is))
    }
}
