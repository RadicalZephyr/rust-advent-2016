use super::{Direction, Instruction};
use std::collections::HashMap;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
pub enum Heading {
    North,
    East,
    South,
    West,
}

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
{
    let mut m = ::std::collections::HashMap::new();
    $(
        m.insert($key, $value);
    )+
        m
}
     };
);

impl Heading {
    pub fn mappings() -> HashMap<Heading, HashMap<Direction, Heading>> {
        use self::Heading::*;
        use super::Direction::*;

        map!(
            North => map!(Left => West, Right => East),
            East => map!(Left => North, Right => South),
            South => map!(Left => East, Right => West),
            West => map!(Left => South, Right => North)
        )
    }

    pub fn turn(h: Heading, d: Direction) -> Self {
        *Self::mappings().get(&h).unwrap().get(&d).unwrap()
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Location {
    x: i64,
    y: i64,
    heading: Heading,
}

pub fn travel(heading: Heading, distance: u8) -> (i64, i64) {
    use self::Heading::*;
    let distance = distance as i64;
    match heading {
        North => (0, distance),
        East => (distance, 0),
        South => (0, -distance),
        West => (-distance, 0),
    }
}

impl Location {
    pub fn new() -> Self {
        Location {
            x: 0,
            y: 0,
            heading: Heading::North,
        }
    }

    pub fn follow_instruction(self, i: Instruction) -> Self {
        let new_heading = Heading::turn(self.heading, i.direction);
        let (dx, dy) = travel(new_heading, i.distance);
        Location {
            x: 0 + dx,
            y: 0 + dy,
            heading: new_heading,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use day_1::*;

    #[test]
    pub fn test_follow_instruction() {
        let l = Location::new();
        let i = Instruction::new(Direction::Left, 1);
        let actual_location = l.follow_instruction(i);
        let expected_location = Location {
            x: -1,
            y: 0,
            heading: Heading::West,
        };
        assert_eq!(actual_location, expected_location);
    }
}
