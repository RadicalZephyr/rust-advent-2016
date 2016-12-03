pub mod parse;
pub mod navigation;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Instruction {
    direction: Direction,
    distance: u8,
}

impl Instruction {
    pub fn new(direction: Direction, distance: u8) -> Self {
        Instruction {
            direction: direction,
            distance: distance,
        }
    }
}
