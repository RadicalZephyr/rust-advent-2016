use std::str;
use std::str::FromStr;

use nom::{alpha, digit};

use super::Room;

named!(pub checksum<&str>,
       delimited!(tag!("["),
                  map_res!(
                      alpha,
                      str::from_utf8
                  ),
                  tag!("]"))
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
           chain!(name: name ~
                  tag!("-") ~
                  sector_id: sector_id ~
                  checksum: checksum,
                  || (sector_id, checksum, name)),
           Room::from_tuple
       )
);

named!(pub rooms<Vec<Room> >,
       many1!(ws!(room))
);

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Room;

    use nom::IResult;

    #[test]
    fn test_name_part() {
        assert_eq!(name_part(b"ab"), IResult::Done(&b""[..], "ab"));
    }

    #[test]
    fn test_name() {
        assert_eq!(name(b"ab-cd"), IResult::Done(&b""[..], vec!["ab", "cd"]));
        assert_eq!(name(b"cd-de-ab"), IResult::Done(&b""[..], vec!["cd", "de", "ab"]));
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum(b"[checksum]"), IResult::Done(&b""[..], "checksum"));
        assert_eq!(checksum(b"[thing]"), IResult::Done(&b""[..], "thing"));
    }

    #[test]
    fn test_room() {
        assert_eq!(room(b"ab-123[chk]"),
                   IResult::Done(&b""[..], Room::from_tuple((123, "chk", vec!["ab"]))));
        assert_eq!(room(b"ab-cd-543[def]"),
                   IResult::Done(&b""[..], Room::from_tuple((543, "def", vec!["ab", "cd"]))));
    }

    #[test]
    fn test_rooms() {
        assert_eq!(rooms(b"ab-123[chk]\nab-cd-543[def]"),
                   IResult::Done(&b""[..], vec![
                       Room::from_tuple((123, "chk", vec!["ab"])),
                       Room::from_tuple((543, "def", vec!["ab", "cd"]))
                   ]));
    }
}
