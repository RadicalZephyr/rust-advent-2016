use std::str;
use std::str::FromStr;

use nom::digit;

use super::Triangle;

named!(pub side<u16>,
       map_res!(
           map_res!(
               ws!(digit),
               str::from_utf8
           ),
           FromStr::from_str
       )
);

named!(pub triangle<Triangle>,
       map!(
           tuple!(side, side, side),
           Triangle::new
       )
);

named!(pub triangles_list<Vec<Triangle> >,
       many1!(triangle)
);

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Triangle;

    use nom::IResult;

    #[test]
    fn test_side() {
        assert_eq!(side(b"10"), IResult::Done(&b""[..], 10));
        assert_eq!(side(b"1   "), IResult::Done(&b""[..], 1));
        assert_eq!(side(b"   3"), IResult::Done(&b""[..], 3));
        assert_eq!(side(b"   33  "), IResult::Done(&b""[..], 33));
    }

    #[test]
    fn test_triangle() {
        assert_eq!(triangle(b"1 1 1"), IResult::Done(&b""[..], Triangle::new((1,1,1))));
        assert_eq!(triangle(b"1   2    1"), IResult::Done(&b""[..], Triangle::new((1,2,1))));
    }

    #[test]
    fn test_triangles_list() {
        assert_eq!(triangles_list(b"1 1 1\n3 3 3"),
                   IResult::Done(&b""[..],
                                 vec![
                                     Triangle::new((1,1,1)),
                                     Triangle::new((3,3,3))
                                 ]));
    }
}
