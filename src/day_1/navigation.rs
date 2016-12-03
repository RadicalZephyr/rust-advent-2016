use super::{Direction, Instruction};
use std::collections::{HashMap, HashSet};

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

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
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

#[derive(Debug)]
struct TrackingLocationReduction {
    hq_location: Option<Location>,
    visited: HashSet<Location>,
    current: Location,
}

impl TrackingLocationReduction {
    pub fn new(location: Location) -> Self {
        let mut visited = HashSet::new();
        visited.insert(location.clone());
        TrackingLocationReduction {
            hq_location: None,
            visited: visited,
            current: location,
        }
    }

    pub fn follow_instruction(self, i: Instruction) -> Self {
        let mut visited = self.visited;
        let start = self.current.clone();
        let next_location = self.current.follow_instruction(i);
        let hq_location = match self.hq_location {
            Some(l) => Some(l),
            None => {
                if visited.contains(&next_location) {
                    Some(next_location.clone())
                } else {
                    None
                }
            }
        };
        visited.insert(next_location.clone());

        TrackingLocationReduction {
            hq_location: hq_location,
            visited: visited,
            current: next_location,
        }
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
            x: self.x + dx,
            y: self.y + dy,
            heading: new_heading,
        }
    }

    pub fn follow_all_instructions(self, instructions: Vec<Instruction>) -> Self {
        instructions.into_iter().fold(self, Self::follow_instruction)
    }

    pub fn first_repeated_location(self, instructions: Vec<Instruction>) -> Self {
        let result = instructions.into_iter().fold(TrackingLocationReduction::new(self),
                                                   TrackingLocationReduction::follow_instruction);
        match result.hq_location {
            Some(l) => l,
            None => panic!("No already visited location found."),
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

    #[test]
    pub fn test_follow_many_instructions() {
        let l = Location::new();
        let i1 = Instruction::new(Direction::Left, 1);
        let i2 = Instruction::new(Direction::Right, 1);
        let is = vec![i1, i2];
        let final_location = l.follow_all_instructions(is);
        let expected_location = Location {
            x: -1,
            y: 1,
            heading: Heading::North,
        };
        assert_eq!(final_location, expected_location);
    }
}
