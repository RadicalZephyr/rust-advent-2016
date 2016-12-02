use std::str;
use std::str::FromStr;

use nom::digit;

use super::{Direction, Instruction};

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

named!(pub delim, eat_separator!(&b", "[..]));

named!(pub instructions<Vec<Instruction> >,
       separated_nonempty_list!(delim, instruction)
);

#[cfg(test)]
pub mod test {
    use super::*;
    use super::super::*;

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
        assert_eq!(instructions(b"L1, R4"), IResult::Done(&b""[..], is.clone()));
        assert_eq!(instructions(b"L1,R4"), IResult::Done(&b""[..], is.clone()));
        assert_eq!(instructions(b"L1 R4"), IResult::Done(&b""[..], is.clone()));
    }
}
