use std::str;
use std::str::FromStr;

use super::Direction;

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use super::Direction::*;
        match s {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => Err(()),
        }
    }
}

named!(pub direction<Direction>,
       map_res!(
           map_res!(
               alt!(tag!("U") | tag!("D") | tag!("L") | tag!("R")),
               str::from_utf8
           ),
           FromStr::from_str
       )
);

named!(pub instructions<Vec<Direction> >,
       many1!(direction)
);

named!(delim, eat_separator!(&b"\n"[..]));

named!(pub instructions_list<Vec<Vec<Direction> > >,
    separated_nonempty_list!(delim, instructions)
);

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Direction::*;

    use nom::IResult;

    #[test]
    fn test_direction() {
        assert_eq!(direction(b"U"), IResult::Done(&b""[..], Up));
        assert_eq!(direction(b"D"), IResult::Done(&b""[..], Down));
        assert_eq!(direction(b"L"), IResult::Done(&b""[..], Left));
        assert_eq!(direction(b"R"), IResult::Done(&b""[..], Right));
    }

    #[test]
    fn test_instructions() {
        assert_eq!(instructions(b"UUL"), IResult::Done(&b""[..], vec![Up, Up, Left]));
        assert_eq!(instructions(b"LDRU"), IResult::Done(&b""[..], vec![Left, Down, Right, Up]));
    }

    #[test]
    fn test_instructions_list() {
        assert_eq!(instructions_list(b"UUL\nLDRU"),
                   IResult::Done(&b""[..],
                                 vec![
                                     vec![Up, Up, Left],
                                     vec![Left, Down, Right, Up]
                                 ]));
    }
}
