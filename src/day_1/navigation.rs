use super::{Direction, Instruction};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

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

    pub fn turn(&self, d: Direction) -> Self {
        *Self::mappings().get(self).unwrap().get(&d).unwrap()
    }
}

fn travel(heading: Heading, distance: u8) -> (i64, i64) {
    use self::Heading::*;
    let distance = distance as i64;
    match heading {
        North => (0, distance),
        East => (distance, 0),
        South => (0, -distance),
        West => (-distance, 0),
    }
}

fn travel_iter(heading: Heading, distance: u8) -> Vec<(i64, i64)> {
    use self::Heading::*;
    let distance = distance as i64;
    match heading {
        North => (0..(distance + 1)).into_iter().map(|i| (0, i)).collect(),
        East => (0..(distance + 1)).into_iter().map(|i| (i, 0)).collect(),
        South => (0..(-(distance + 1))).into_iter().map(|i| (0, i)).collect(),
        West => (0..(-(distance + 1))).into_iter().map(|i| (i, 0)).collect(),
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Location {
    x: i64,
    y: i64,
    heading: Heading,
}

impl PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Location {}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
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

    pub fn turn_for(&self, instruction: &Instruction) -> Self {
        Location {
            x: self.x,
            y: self.y,
            heading: self.heading.turn(instruction.direction),
        }
    }

    pub fn walk_for(&self, instruction: &Instruction) -> Self {
        let (dx, dy) = travel(self.heading, instruction.distance);
        Location {
            x: self.x + dx,
            y: self.y + dy,
            heading: self.heading,
        }
    }

    pub fn locations_walked_through_for(&self, instruction: &Instruction) -> Vec<Self> {
        travel_iter(self.heading, instruction.distance)
            .into_iter()
            .map(|(dx, dy)| {
                Location {
                    x: self.x + dx,
                    y: self.y + dy,
                    heading: self.heading,
                }
            })
            .collect()
    }

    pub fn follow_instruction(self, i: Instruction) -> Self {
        self.turn_for(&i).walk_for(&i)
    }

    pub fn follow_all_instructions(self, instructions: Vec<Instruction>) -> Self {
        instructions.into_iter().fold(self, Self::follow_instruction)
    }

    pub fn first_repeated_location(self, instructions: Vec<Instruction>) -> Option<Self> {
        let result = instructions.into_iter().fold(TrackingLocationReduction::new(self),
                                                   TrackingLocationReduction::follow_instruction);
        result.hq_location
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
        visited.insert(location);
        TrackingLocationReduction {
            hq_location: None,
            visited: visited,
            current: location,
        }
    }

    pub fn travel_along_path(&mut self, instruction: Instruction) -> (Location, Option<Location>) {
        let correct_heading = self.current.turn_for(&instruction);
        let next_location = correct_heading.walk_for(&instruction);
        let mut hq_location = None;
        for location in correct_heading.locations_walked_through_for(&instruction) {
            hq_location = match hq_location {
                Some(l) => Some(l),
                None => {
                    if self.visited.contains(&location) {
                        Some(location)
                    } else {
                        None
                    }
                }
            };
            self.visited.insert(location);
        }

        (next_location, hq_location)
    }

    pub fn follow_instruction(mut self, instruction: Instruction) -> Self {
        let (next_location, hq_location) = self.travel_along_path(instruction);

        TrackingLocationReduction {
            hq_location: hq_location,
            visited: self.visited,
            current: next_location,
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
        let instructions = vec![
            Instruction::new(Direction::Left, 1),
            Instruction::new(Direction::Right, 1)
        ];
        let final_location = l.follow_all_instructions(instructions);
        let expected_location = Location {
            x: -1,
            y: 1,
            heading: Heading::North,
        };
        assert_eq!(final_location, expected_location);
    }

    #[test]
    pub fn test_follow_visiting_all() {
        let l = Location::new();
        let instructions = vec![
            Instruction::new(Direction::Right, 8),
            Instruction::new(Direction::Right, 4),
            Instruction::new(Direction::Right, 4),
            Instruction::new(Direction::Right, 8),
        ];
        let hq_location = l.first_repeated_location(instructions);
        let expected_location = Location {
            x: 4,
            y: 0,
            heading: Heading::North,
        };
        assert_eq!(hq_location, Some(expected_location));
    }
}
