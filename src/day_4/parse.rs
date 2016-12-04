use std::str;
use std::str::FromStr;

use nom::{alpha, digit};

use super::Room;

named!(pub chkstring<&str>,
       map_res!(
           alpha,
           str::from_utf8
       )
);

named!(pub checksum<&str>,
       delimited!(tag!("["), call!(chkstring), tag!("]"))
);

named!(pub sector_id<u16>,
       map_res!(
           map_res!(
               digit,
               str::from_utf8
           ),
           FromStr::from_str
       )
);

named!(pub delim, eat_separator!(&b"-"[..]));

named!(pub name_part<&str>,
       map_res!(
           alpha,
           str::from_utf8
       )
);

named!(pub name<Vec<&str> >,
       separated_nonempty_list!(delim, name_part)
);

named!(pub room<Room>,
       map!(
           tuple!(name, sector_id, checksum),
           Room::from_tuple
       )
);

named!(pub rooms<Vec<Room> >,
       many1!(room)
);

#[cfg(test)]
mod tests {
    use super::*;

    use nom::IResult;

    #[test]
    fn test_name_part() {
        assert_eq!(name_part(b"ab"), IResult::Done(&b""[..], "ab"));
    }

    #[test]
    fn test_name() {
        assert_eq!(name(b"ab-cd"),
                   IResult::Done(&b""[..],
                                 vec![
                                     "ab",
                                     "cd"
                                 ]));
    }
}
