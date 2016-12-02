pub mod parse;
pub mod navigation;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Direction {
    L,
    R,
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Instruction {
    direction: Direction,
    distance: u8,
}
