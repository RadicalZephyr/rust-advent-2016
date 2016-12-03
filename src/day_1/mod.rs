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
